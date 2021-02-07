// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
//////// DO NOT CHANGE THIS FILE! ////////
// Change the json schema instead

package dividend

import "github.com/iotaledger/wasp/packages/vm/wasmlib"

type Member struct {
	Address *wasmlib.ScAddress // address of dividend recipient
	Factor  int64              // relative division factor
}

func EncodeMember(o *Member) []byte {
	return wasmlib.NewBytesEncoder().
		Address(o.Address).
		Int(o.Factor).
		Data()
}

func DecodeMember(bytes []byte) *Member {
	decode := wasmlib.NewBytesDecoder(bytes)
	data := &Member{}
	data.Address = decode.Address()
	data.Factor = decode.Int()
	return data
}
