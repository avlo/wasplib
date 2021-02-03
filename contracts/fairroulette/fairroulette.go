// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package fairroulette

import "github.com/iotaledger/wasplib/client"

const MaxNumber = 5
const DefaultPlayPeriod = 120

func funcPlaceBet(ctx *client.ScCallContext) {
	amount := ctx.Incoming().Balance(client.IOTA)
	if amount == 0 {
		ctx.Panic("Empty bet...")
	}
	number := ctx.Params().GetInt(ParamNumber).Value()
	if number == 0 {
		ctx.Panic("No number...")
	}
	if number < 1 || number > MaxNumber {
		ctx.Panic("Invalid number...")
	}

	bet := &BetInfo{
		Better: ctx.Caller(),
		Amount: amount,
		Number: number,
	}

	state := ctx.State()
	bets := state.GetBytesArray(VarBets)
	betNr := bets.Length()
	bets.GetBytes(betNr).SetValue(EncodeBetInfo(bet))
	if betNr == 0 {
		playPeriod := state.GetInt(VarPlayPeriod).Value()
		if playPeriod < 10 {
			playPeriod = DefaultPlayPeriod
		}
		ctx.Post(&client.PostRequestParams{
			ContractId: ctx.ContractId(),
			Function:   HFuncLockBets,
			Params:     nil,
			Transfer:   nil,
			Delay:      playPeriod,
		})
	}
}

func funcLockBets(ctx *client.ScCallContext) {
	// can only be sent by SC itself
	if !ctx.From(ctx.ContractId().AsAgentId()) {
		ctx.Panic("Cancel spoofed request")
	}

	// move all current bets to the locked_bets array
	state := ctx.State()
	bets := state.GetBytesArray(VarBets)
	lockedBets := state.GetBytesArray(VarLockedBets)
	nrBets := bets.Length()
	for i := int32(0); i < nrBets; i++ {
		bytes := bets.GetBytes(i).Value()
		lockedBets.GetBytes(i).SetValue(bytes)
	}
	bets.Clear()

	ctx.Post(&client.PostRequestParams{
		ContractId: ctx.ContractId(),
		Function:   HFuncPayWinners,
		Params:     nil,
		Transfer:   nil,
		Delay:      0,
	})
}

func funcPayWinners(ctx *client.ScCallContext) {
	// can only be sent by SC itself
	scId := ctx.ContractId().AsAgentId()
	if !ctx.From(scId) {
		ctx.Panic("Cancel spoofed request")
	}

	winningNumber := ctx.Utility().Random(5) + 1
	state := ctx.State()
	state.GetInt(VarLastWinningNumber).SetValue(winningNumber)

	// gather all winners and calculate some totals
	totalBetAmount := int64(0)
	totalWinAmount := int64(0)
	lockedBets := state.GetBytesArray(VarLockedBets)
	winners := make([]*BetInfo, 0)
	nrBets := lockedBets.Length()
	for i := int32(0); i < nrBets; i++ {
		bet := DecodeBetInfo(lockedBets.GetBytes(i).Value())
		totalBetAmount += bet.Amount
		if bet.Number == winningNumber {
			totalWinAmount += bet.Amount
			winners = append(winners, bet)
		}
	}
	lockedBets.Clear()

	if len(winners) == 0 {
		ctx.Log("Nobody wins!")
		// compact separate bet deposit UTXOs into a single one
		ctx.TransferToAddress(scId.Address(), client.NewScTransfer(client.IOTA, totalBetAmount))
		return
	}

	// pay out the winners proportionally to their bet amount
	totalPayout := int64(0)
	size := len(winners)
	for i := 0; i < size; i++ {
		bet := winners[i]
		payout := totalBetAmount * bet.Amount / totalWinAmount
		if payout != 0 {
			totalPayout += payout
			ctx.TransferToAddress(bet.Better.Address(), client.NewScTransfer(client.IOTA, payout))
		}
		text := "Pay " + ctx.Utility().String(payout) +
			" to " + bet.Better.String()
		ctx.Log(text)
	}

	// any truncation left-overs are fair picking for the smart contract
	if totalPayout != totalBetAmount {
		remainder := totalBetAmount - totalPayout
		text := "Remainder is " + ctx.Utility().String(remainder)
		ctx.Log(text)
		ctx.TransferToAddress(scId.Address(), client.NewScTransfer(client.IOTA, remainder))
	}
}

func funcPlayPeriod(ctx *client.ScCallContext) {
	// can only be sent by SC creator
	if !ctx.From(ctx.ContractCreator()) {
		ctx.Panic("Cancel spoofed request")
	}

	playPeriod := ctx.Params().GetInt(ParamPlayPeriod).Value()
	if playPeriod < 10 {
		ctx.Panic("Invalid play period...")
	}

	ctx.State().GetInt(VarPlayPeriod).SetValue(playPeriod)
}
