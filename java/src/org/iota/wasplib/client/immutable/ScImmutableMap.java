package org.iota.wasplib.client.immutable;

import org.iota.wasplib.client.Host;
import org.iota.wasplib.client.Keys;
import org.iota.wasplib.client.ScType;

public class ScImmutableMap {
	int objId;

	public ScImmutableMap(int objId) {
		this.objId = objId;
	}

	public ScImmutableAddress GetAddress(String key) {
		return new ScImmutableAddress(objId, Host.GetKeyId(key));
	}

	public ScImmutableAddressArray GetAddressArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScImmutableAddressArray(arrId);
	}

	public ScImmutableBytes GetBytes(String key) {
		return new ScImmutableBytes(objId, Host.GetKeyId(key));
	}

	public ScImmutableBytesArray GetBytesArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScImmutableBytesArray(arrId);
	}

	public ScImmutableColor GetColor(String key) {
		return new ScImmutableColor(objId, Host.GetKeyId(key));
	}

	public ScImmutableColorArray GetColorArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScImmutableColorArray(arrId);
	}

	public ScImmutableInt GetInt(String key) {
		return new ScImmutableInt(objId, Host.GetKeyId(key));
	}

	public ScImmutableIntArray GetIntArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_INT_ARRAY);
		return new ScImmutableIntArray(arrId);
	}

	public ScImmutableKeyMap GetKeyMap(String key) {
		int mapId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_MAP);
		return new ScImmutableKeyMap(mapId);
	}

	public ScImmutableMap GetMap(String key) {
		int mapId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_MAP);
		return new ScImmutableMap(mapId);
	}

	public ScImmutableMapArray GetMapArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_MAP_ARRAY);
		return new ScImmutableMapArray(arrId);
	}

	public ScImmutableRequestId GetRequestId(String key) {
		return new ScImmutableRequestId(objId, Host.GetKeyId(key));
	}

	public ScImmutableRequestIdArray GetRequestIdArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScImmutableRequestIdArray(arrId);
	}

	public ScImmutableString GetString(String key) {
		return new ScImmutableString(objId, Host.GetKeyId(key));
	}

	public ScImmutableStringArray GetStringArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_STRING_ARRAY);
		return new ScImmutableStringArray(arrId);
	}

	public ScImmutableTxHash GetTxHash(String key) {
		return new ScImmutableTxHash(objId, Host.GetKeyId(key));
	}

	public ScImmutableTxHashArray GetTxHashArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScImmutableTxHashArray(arrId);
	}

	public int Length() {
		return (int) Host.GetInt(objId, Keys.KeyLength());
	}
}
