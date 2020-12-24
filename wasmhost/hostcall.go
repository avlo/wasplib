// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package wasmhost

import (
	"fmt"
	"github.com/mr-tron/base58"
)

type HostCall struct {
	HostMap
	chain    []byte
	contract string
	function string
	delay    int64
}

func NewHostCall(host *SimpleWasmHost, keyId int32) *HostCall {
	return &HostCall{HostMap: *NewHostMap(host, keyId)}
}

func (a *HostCall) call() {
	host := a.host

	root := host.FindObject(1)
	savedCaller := root.GetString(KeyCaller)
	scId := host.FindSubObject(nil, KeyContract, OBJTYPE_MAP).GetString(KeyId)
	root.SetString(KeyCaller, scId)

	requestParams := host.FindSubObject(nil, KeyParams, OBJTYPE_MAP)
	savedParams := NewHostMap(a.host, 0)
	requestParams.(*HostMap).CopyDataTo(savedParams)
	requestParams.SetInt(KeyLength, 0)
	params := host.FindSubObject(a, KeyParams, OBJTYPE_MAP)
	params.(*HostMap).CopyDataTo(requestParams)

	fmt.Printf("    Call function: %v\n", a.function)
	err := host.RunScFunction(a.function)
	if err != nil {
		fmt.Printf("FAIL: Request function %s: %v\n", a.function, err)
		a.Error(err.Error())
	}

	requestParams.SetInt(KeyLength, 0)
	savedParams.CopyDataTo(requestParams)
	root.SetString(KeyCaller, savedCaller)
}

func (a *HostCall) SetBytes(keyId int32, value []byte) {
	key := string(a.host.GetKeyFromId(keyId))
	a.host.TraceAll("Call.SetBytes %s = %s", key, base58.Encode(value))
	a.HostMap.SetBytes(keyId, value)
	if key == "chain" {
		a.chain = value
		return
	}
}

func (a *HostCall) SetInt(keyId int32, value int64) {
	key := string(a.host.GetKeyFromId(keyId))
	a.host.TraceAll("Call.SetInt %s = %d\n", key, value)
	a.HostMap.SetInt(keyId, value)
	if key != "delay" {
		return
	}
	if a.contract == "" {
		// call to self, immediately executed
		a.call()
		return
	}
	panic("Call.SetInt: call to other contract not implemented yet")
	//TODO take return values from json
}

func (a *HostCall) SetString(keyId int32, value string) {
	key := string(a.host.GetKeyFromId(keyId))
	a.host.TraceAll("Call.SetString %s = %s\n", key, value)
	a.HostMap.SetString(keyId, value)
	if key == "contract" {
		a.contract = value
		return
	}
	if key == "function" {
		a.function = value
		return
	}
}
