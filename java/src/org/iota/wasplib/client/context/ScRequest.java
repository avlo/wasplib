package org.iota.wasplib.client.context;

import org.iota.wasplib.client.hashtypes.ScAddress;
import org.iota.wasplib.client.hashtypes.ScColor;
import org.iota.wasplib.client.hashtypes.ScRequestId;
import org.iota.wasplib.client.hashtypes.ScTxHash;
import org.iota.wasplib.client.immutable.ScImmutableColorArray;
import org.iota.wasplib.client.immutable.ScImmutableMap;

public class ScRequest {
	ScImmutableMap request;

	ScRequest(ScImmutableMap request) {
		this.request = request;
	}

	public ScAddress Address() {
		return request.GetAddress("address").Value();
	}

	public long Balance(ScColor color) {
		return request.GetKeyMap("balance").GetInt(color.toBytes()).Value();
	}

	public ScImmutableColorArray Colors() {
		return request.GetColorArray("colors");
	}

	public Boolean From(ScAddress originator) {
		return From(originator);
	}

	public ScRequestId Id() {
		return request.GetRequestId("id").Value();
	}

	public ScColor MintedColor() {
		return request.GetColor("hash").Value();
	}

	public ScImmutableMap Params() {
		return request.GetMap("params");
	}

	public long Timestamp() {
		return request.GetInt("timestamp").Value();
	}

	public ScTxHash TxHash() {
		return request.GetTxHash("hash").Value();
	}
}
