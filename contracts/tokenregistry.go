package main

import (
	"github.com/iotaledger/wasplib/client"
)

type TokenInfo struct {
	supply      int64
	mintedBy    *client.ScAddress
	owner       *client.ScAddress
	created     int64
	updated     int64
	description string
	userDefined string
}

func main() {
}

//export onLoad
func onLoadTokenRegistry() {
	exports := client.NewScExports()
	exports.Add("mintSupply")
	exports.Add("updateMetadata")
	exports.Add("transferOwnership")
}

//export mintSupply
func mintSupply() {
	sc := client.NewScContext()
	request := sc.Request()
	color := request.MintedColor()
	state := sc.State()
	registry := state.GetKeyMap("registry").GetBytes(color.Bytes())
	if registry.Exists() {
		sc.Log("TokenRegistry: Color already exists")
		return
	}
	params := request.Params()
	token := &TokenInfo{
		supply:      request.Balance(color),
		mintedBy:    request.Address(),
		owner:       request.Address(),
		created:     request.Timestamp(),
		updated:     request.Timestamp(),
		description: params.GetString("dscr").Value(),
		userDefined: params.GetString("ud").Value(),
	}
	if token.supply <= 0 {
		sc.Log("TokenRegistry: Insufficient supply")
		return
	}
	if len(token.description) == 0 {
		token.description += "no dscr"
	}
	bytes := encodeTokenInfo(token)
	registry.SetValue(bytes)
	colors := state.GetColorArray("colorList")
	colors.GetColor(colors.Length()).SetValue(color)
}

//export updateMetadata
func updateMetadata() {
	//sc := client.NewScContext()
	//TODO
}

//export transferOwnership
func transferOwnership() {
	//sc := client.NewScContext()
	//TODO
}

func decodeTokenInfo(bytes []byte) *TokenInfo {
	decoder := client.NewBytesDecoder(bytes)
	data := &TokenInfo{}
	data.supply = decoder.Int()
	data.mintedBy = decoder.Address()
	data.owner = decoder.Address()
	data.created = decoder.Int()
	data.updated = decoder.Int()
	data.description = decoder.String()
	data.userDefined = decoder.String()
	return data
}

func encodeTokenInfo(token *TokenInfo) []byte {
	return client.NewBytesEncoder().
		Int(token.supply).
		Address(token.mintedBy).
		Address(token.owner).
		Int(token.created).
		Int(token.updated).
		String(token.description).
		String(token.userDefined).
		Data()
}
