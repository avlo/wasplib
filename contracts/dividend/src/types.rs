// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
//////// DO NOT CHANGE THIS FILE! ////////
// Change the json schema instead

#![allow(dead_code)]

use wasmlib::*;

//@formatter:off
pub struct Member {
    pub address: ScAddress, // address of dividend recipient
    pub factor:  i64,       // relative division factor
}
//@formatter:on

pub fn encode_member(o: &Member) -> Vec<u8> {
    let mut encode = BytesEncoder::new();
    encode.address(&o.address);
    encode.int(o.factor);
    return encode.data();
}

pub fn decode_member(bytes: &[u8]) -> Member {
    let mut decode = BytesDecoder::new(bytes);
    Member {
        address: decode.address(),
        factor: decode.int(),
    }
}