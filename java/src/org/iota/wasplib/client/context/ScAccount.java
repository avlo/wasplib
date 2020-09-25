package org.iota.wasplib.client.context;

import org.iota.wasplib.client.immutable.ScImmutableMap;
import org.iota.wasplib.client.immutable.ScImmutableStringArray;

public class ScAccount {
	ScImmutableMap account;

	ScAccount(ScImmutableMap account) {
		this.account = account;
	}

	public long Balance(String color) {
		String key = color.isEmpty() ? "iota" : color;
		return account.GetMap("balance").GetInt(key).Value();
	}

	public ScImmutableStringArray Colors() {
		return account.GetStringArray("colors");
	}
}
