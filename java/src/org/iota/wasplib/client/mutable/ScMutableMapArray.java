package org.iota.wasplib.client.mutable;

import org.iota.wasplib.client.Host;
import org.iota.wasplib.client.Keys;
import org.iota.wasplib.client.ScType;
import org.iota.wasplib.client.immutable.ScImmutableMapArray;

public class ScMutableMapArray {
	int objId;

	public ScMutableMapArray(int objId) {
		this.objId = objId;
	}

	public void Clear() {
		Host.SetInt(objId, Keys.KeyLength(), 0);
	}

	public ScMutableKeyMap GetKeyMap(int index) {
		int mapId = Host.GetObjectId(objId, index, ScType.OBJTYPE_MAP);
		return new ScMutableKeyMap(mapId);
	}

	public ScMutableMap GetMap(int index) {
		int mapId = Host.GetObjectId(objId, index, ScType.OBJTYPE_MAP);
		return new ScMutableMap(mapId);
	}

	public ScImmutableMapArray Immutable() {
		return new ScImmutableMapArray(objId);
	}

	public int Length() {
		return (int) Host.GetInt(objId, Keys.KeyLength());
	}
}
