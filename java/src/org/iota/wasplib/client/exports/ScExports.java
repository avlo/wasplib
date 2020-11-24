// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.wasplib.client.exports;

import org.iota.wasplib.client.context.ScCallContext;
import org.iota.wasplib.client.context.ScViewContext;
import org.iota.wasplib.client.mutable.ScMutableMap;
import org.iota.wasplib.client.mutable.ScMutableStringArray;

import java.util.ArrayList;

public class ScExports {
	private static final ArrayList<ScCall> calls = new ArrayList<>();
	private static final ArrayList<ScView> views = new ArrayList<>();

	ScMutableStringArray exports;

	public ScExports() {
		ScMutableMap root = new ScMutableMap(1);
		exports = root.GetStringArray("exports");
	}

	//export sc_call_entrypoint
	static void scCallEntrypoint(int index) {
		if ((index & 0x8000) != 0) {
			ScView view = views.get(index & 0x7fff);
			view.call(new ScViewContext());
			return;
		}
		calls.get(index).call(new ScCallContext());
	}

	public static void nothing(ScCallContext sc) {
		sc.Log("Doing nothing as requested. Oh, wait...");
	}

	public void AddCall(String name, ScCall f) {
		int index = calls.size();
		calls.add(f);
		exports.GetString(index).SetValue(name);
	}

	public void AddView(String name, ScView f) {
		int index = views.size();
		views.add(f);
		exports.GetString(index | 0x8000).SetValue(name);
	}
}