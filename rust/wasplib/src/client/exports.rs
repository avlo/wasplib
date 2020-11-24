// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// encapsulates standard host entities into a simple interface

use super::context::*;
use super::mutable::*;

static mut CALLS: Vec<fn(&ScCallContext)> = vec![];
static mut VIEWS: Vec<fn(&ScViewContext)> = vec![];

#[no_mangle]
fn sc_call_entrypoint(index: i32) {
    unsafe {
        if (index & 0x8000) != 0 {
            VIEWS[(index & 0x7fff) as usize](&ROOT_VIEW_CONTEXT);
            return;
        }

        CALLS[index as usize](&ROOT_CALL_CONTEXT);
    }
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

pub struct ScExports {
    exports: ScMutableStringArray,
}

impl ScExports {
    pub fn new() -> ScExports {
        let root = ScMutableMap::new(1);
        ScExports { exports: root.get_string_array("exports") }
    }

    pub fn add_call(&self, name: &str, f: fn(&ScCallContext)) {
        unsafe {
            let index = CALLS.len() as i32;
            CALLS.push(f);
            self.exports.get_string(index).set_value(name);
        }
    }

    pub fn add_view(&self, name: &str, f: fn(&ScViewContext)) {
        unsafe {
            let index = VIEWS.len() as i32;
            VIEWS.push(f);
            self.exports.get_string(index | 0x8000).set_value(name);
        }
    }

    pub fn nothing(sc: &ScCallContext) {
        sc.log("Doing nothing as requested. Oh, wait...");
    }
}
