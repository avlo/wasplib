// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
//////// DO NOT CHANGE THIS FILE! ////////
// Change the json schema instead

package tokenregistry

import "github.com/iotaledger/wasp/packages/vm/wasmlib"

func OnLoad() {
	exports := wasmlib.NewScExports()
	exports.AddCall(FuncMintSupply, funcMintSupply)
	exports.AddCall(FuncTransferOwnership, funcTransferOwnership)
	exports.AddCall(FuncUpdateMetadata, funcUpdateMetadata)
	exports.AddView(ViewGetInfo, viewGetInfo)
}
