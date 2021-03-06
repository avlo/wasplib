package org.iota.wasplib.client.immutable;

import org.iota.wasplib.client.Host;
import org.iota.wasplib.client.Keys;

public class ScImmutableIntArray {
	int objId;

	public ScImmutableIntArray(int objId) {
		this.objId = objId;
	}

	public ScImmutableInt GetInt(int index) {
		return new ScImmutableInt(objId, index);
	}

	public int Length() {
		return (int) Host.GetInt(objId, Keys.KeyLength());
	}
}
