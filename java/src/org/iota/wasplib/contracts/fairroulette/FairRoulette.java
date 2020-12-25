// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.wasplib.contracts.fairroulette;

import org.iota.wasplib.client.context.ScCallContext;
import org.iota.wasplib.client.exports.ScExports;
import org.iota.wasplib.client.hashtypes.ScAgent;
import org.iota.wasplib.client.hashtypes.ScColor;
import org.iota.wasplib.client.keys.Key;
import org.iota.wasplib.client.mutable.ScMutableBytesArray;
import org.iota.wasplib.client.mutable.ScMutableMap;

import java.util.ArrayList;

public class FairRoulette {
	private static final Key keyBets = new Key("bets");
	private static final Key keyColor = new Key("color");
	private static final Key keyLastWinningColor = new Key("last_winning_color");
	private static final Key keyLockedBets = new Key("locked_bets");
	private static final Key keyPlayPeriod = new Key("play_period");

	private static final int numColors = 5;
	private static final int defaultPlayPeriod = 120;

	public static void onLoad() {
		ScExports exports = new ScExports();
		exports.AddCall("place_bet", FairRoulette::placeBet);
		exports.AddCall("lock_bets", FairRoulette::lockBets);
		exports.AddCall("pay_winners", FairRoulette::payWinners);
		exports.AddCall("play_period", FairRoulette::playPeriod);
		exports.AddCall("nothing", ScExports::nothing);
	}

	public static void placeBet(ScCallContext sc) {
		long amount = sc.Incoming().Balance(ScColor.IOTA);
		if (amount == 0) {
			sc.Log("Empty bet...");
			return;
		}
		long color = sc.Params().GetInt(keyColor).Value();
		if (color == 0) {
			sc.Log("No color...");
			return;
		}
		if (color < 1 || color > numColors) {
			sc.Log("Invalid color...");
			return;
		}

		BetInfo bet = new BetInfo();
		{
			bet.better = sc.Caller();
			bet.amount = amount;
			bet.color = color;
		}

		ScMutableMap state = sc.State();
		ScMutableBytesArray bets = state.GetBytesArray(keyBets);
		int betNr = bets.Length();
		bets.GetBytes(betNr).SetValue(BetInfo.encode(bet));
		if (betNr == 0) {
			long playPeriod = state.GetInt(keyPlayPeriod).Value();
			if (playPeriod < 10) {
				playPeriod = defaultPlayPeriod;
			}
			sc.Post("lock_bets").Post(playPeriod);
		}
	}

	public static void lockBets(ScCallContext sc) {
		// can only be sent by SC itself
		if (!sc.From(sc.Contract().Id())) {
			sc.Log("Cancel spoofed request");
			return;
		}

		// move all current bets to the locked_bets array
		ScMutableMap state = sc.State();
		ScMutableBytesArray bets = state.GetBytesArray(keyBets);
		ScMutableBytesArray lockedBets = state.GetBytesArray(keyLockedBets);
		int nrBets = bets.Length();
		for (int i = 0; i < nrBets; i++) {
			byte[] bytes = bets.GetBytes(i).Value();
			lockedBets.GetBytes(i).SetValue(bytes);
		}
		bets.Clear();

		sc.Post("pay_winners").Post(0);
	}

	public static void payWinners(ScCallContext sc) {
		// can only be sent by SC itself
		ScAgent scId = sc.Contract().Id();
		if (!sc.From(scId)) {
			sc.Log("Cancel spoofed request");
			return;
		}

		long winningColor = sc.Utility().Random(5) + 1;
		ScMutableMap state = sc.State();
		state.GetInt(keyLastWinningColor).SetValue(winningColor);

		// gather all winners and calculate some totals
		long totalBetAmount = 0;
		long totalWinAmount = 0;
		ScMutableBytesArray lockedBets = state.GetBytesArray(keyLockedBets);
		ArrayList<BetInfo> winners = new ArrayList<BetInfo>();
		int nrBets = lockedBets.Length();
		for (int i = 0; i < nrBets; i++) {
			BetInfo bet = BetInfo.decode(lockedBets.GetBytes(i).Value());
			totalBetAmount += bet.amount;
			if (bet.color == winningColor) {
				totalWinAmount += bet.amount;
				winners.add(bet);
			}
		}
		lockedBets.Clear();

		if (winners.size() == 0) {
			sc.Log("Nobody wins!");
			// compact separate UTXOs into a single one
			sc.Transfer(scId, ScColor.IOTA, totalBetAmount);
			return;
		}

		// pay out the winners proportionally to their bet amount
		int totalPayout = 0;
		int size = winners.size();
		String text;
		for (int i = 0; i < size; i++) {
			BetInfo bet = winners.get(i);
			long payout = totalBetAmount * bet.amount / totalWinAmount;
			if (payout != 0) {
				totalPayout += payout;
				sc.Transfer(bet.better, ScColor.IOTA, payout);
			}
			text = "Pay " + payout + " to " + bet.better;
			sc.Log(text);
		}

		// any truncation left-overs are fair picking for the smart contract
		if (totalPayout != totalBetAmount) {
			long remainder = totalBetAmount - totalPayout;
			text = "Remainder is " + remainder;
			sc.Log(text);
			sc.Transfer(scId, ScColor.IOTA, remainder);
		}
	}

	public static void playPeriod(ScCallContext sc) {
		// can only be sent by SC owner
		if (!sc.From(sc.Contract().Owner())) {
			sc.Log("Cancel spoofed request");
			return;
		}

		long playPeriod = sc.Params().GetInt(keyPlayPeriod).Value();
		if (playPeriod < 10) {
			sc.Log("Invalid play period...");
			return;
		}

		sc.State().GetInt(keyPlayPeriod).SetValue(playPeriod);
	}
}