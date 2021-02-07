// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
//////// DO NOT CHANGE THIS FILE! ////////
// Change the json schema instead

use helloworld::*;
use schema::*;
use wasmlib::*;

mod helloworld;
mod schema;

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_call(FUNC_HELLO_WORLD, func_hello_world);
    exports.add_view(VIEW_GET_HELLO_WORLD, view_get_hello_world);
}
