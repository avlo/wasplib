package org.iota.wasplib.client.mutable;

import org.iota.wasplib.client.Host;
import org.iota.wasplib.client.Keys;
import org.iota.wasplib.client.ScType;
import org.iota.wasplib.client.immutable.ScImmutableMap;

public class ScMutableMap {
	int objId;

	public ScMutableMap(int objId) {
		this.objId = objId;
	}

	public void Clear() {
		Host.SetInt(objId, Keys.KeyLength(), 0);
	}

	public ScMutableAddress GetAddress(String key) {
		return new ScMutableAddress(objId, Host.GetKeyId(key));
	}

	public ScMutableAddressArray GetAddressArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScMutableAddressArray(arrId);
	}

	public ScMutableBytes GetBytes(String key) {
		return new ScMutableBytes(objId, Host.GetKeyId(key));
	}

	public ScMutableBytesArray GetBytesArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScMutableBytesArray(arrId);
	}

	public ScMutableColor GetColor(String key) {
		return new ScMutableColor(objId, Host.GetKeyId(key));
	}

	public ScMutableColorArray GetColorArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScMutableColorArray(arrId);
	}

	public ScMutableInt GetInt(String key) {
		return new ScMutableInt(objId, Host.GetKeyId(key));
	}

	public ScMutableIntArray GetIntArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_INT_ARRAY);
		return new ScMutableIntArray(arrId);
	}

	public ScMutableKeyMap GetKeyMap(String key) {
		int mapId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_MAP);
		return new ScMutableKeyMap(mapId);
	}

	public ScMutableMap GetMap(String key) {
		int mapId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_MAP);
		return new ScMutableMap(mapId);
	}

	public ScMutableMapArray GetMapArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_MAP_ARRAY);
		return new ScMutableMapArray(arrId);
	}

	public ScMutableRequestId GetRequestId(String key) {
		return new ScMutableRequestId(objId, Host.GetKeyId(key));
	}

	public ScMutableRequestIdArray GetRequestIdArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScMutableRequestIdArray(arrId);
	}

	public ScMutableString GetString(String key) {
		return new ScMutableString(objId, Host.GetKeyId(key));
	}

	public ScMutableStringArray GetStringArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_STRING_ARRAY);
		return new ScMutableStringArray(arrId);
	}

	public ScMutableTxHash GetTxHash(String key) {
		return new ScMutableTxHash(objId, Host.GetKeyId(key));
	}

	public ScMutableTxHashArray GetTxHashArray(String key) {
		int arrId = Host.GetObjectId(objId, Host.GetKeyId(key), ScType.OBJTYPE_BYTES_ARRAY);
		return new ScMutableTxHashArray(arrId);
	}

	public ScImmutableMap Immutable() {
		return new ScImmutableMap(objId);
	}

	public int Length() {
		return (int) Host.GetInt(objId, Keys.KeyLength());
	}
}
