// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryInto;

use wasplib::client::*;
use wasplib::client::host::*;

const KEY_COUNTER: &str = "counter";
const KEY_NUM_REPEATS: &str = "num_repeats";

static mut LOCAL_STATE_MUST_INCREMENT: bool = false;

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_call("init", on_init);
    exports.add_call("increment", increment);
    exports.add_call("increment_call_increment", increment_call_increment);
    exports.add_call("increment_call_increment_recurse5x", increment_call_increment_recurse5x);
    exports.add_call("increment_post_increment", increment_post_increment);
    exports.add_view("increment_view_counter", increment_view_counter);
    exports.add_call("increment_repeat_many", increment_repeat_many);
    exports.add_call("increment_when_must_increment", increment_when_must_increment);
    exports.add_call("increment_local_state_internal_call", increment_local_state_internal_call);
    exports.add_call("increment_local_state_sandbox_call", increment_local_state_sandbox_call);
    exports.add_call("increment_local_state_post", increment_local_state_post);
    exports.add_call("nothing", ScExports::nothing);
    exports.add_call("test", test);
    exports.add_call("state_test", state_test);
    exports.add_view("state_check", state_check);
    exports.add_call("results_test", results_test);
    exports.add_view("results_check", results_check);
}

fn on_init(ctx: &ScCallContext) {
    let counter = ctx.params().get_int(KEY_COUNTER).value();
    if counter == 0 {
        return;
    }
    ctx.state().get_int(KEY_COUNTER).set_value(counter);
}

fn increment(ctx: &ScCallContext) {
    let counter = ctx.state().get_int(KEY_COUNTER);
    counter.set_value(counter.value() + 1);
}

fn increment_call_increment(ctx: &ScCallContext) {
    let counter = ctx.state().get_int(KEY_COUNTER);
    let value = counter.value();
    counter.set_value(value + 1);
    if value == 0 {
        ctx.call(ctx.contract_id().hname(), Hname::new("increment_call_increment"), None, None);
    }
}

fn increment_call_increment_recurse5x(ctx: &ScCallContext) {
    let counter = ctx.state().get_int(KEY_COUNTER);
    let value = counter.value();
    counter.set_value(value + 1);
    if value < 5 {
        ctx.call(ctx.contract_id().hname(), Hname::new("increment_call_increment_recurse5x"), None, None);
    }
}

fn increment_post_increment(ctx: &ScCallContext) {
    let counter = ctx.state().get_int(KEY_COUNTER);
    let value = counter.value();
    counter.set_value(value + 1);
    if value == 0 {
        ctx.post(&PostRequestParams {
            contract: ctx.contract_id(),
            function: Hname::new("increment_post_increment"),
            params: None,
            transfer: None,
            delay: 0,
        });
    }
}

fn increment_view_counter(ctx: &ScViewContext) {
    let counter = ctx.state().get_int(KEY_COUNTER).value();
    ctx.results().get_int(KEY_COUNTER).set_value(counter);
}

fn increment_repeat_many(ctx: &ScCallContext) {
    let counter = ctx.state().get_int(KEY_COUNTER);
    let value = counter.value();
    counter.set_value(value + 1);
    let state_repeats = ctx.state().get_int(KEY_NUM_REPEATS);
    let mut repeats = ctx.params().get_int(KEY_NUM_REPEATS).value();
    if repeats == 0 {
        repeats = state_repeats.value();
        if repeats == 0 {
            return;
        }
    }
    state_repeats.set_value(repeats - 1);
    ctx.post(&PostRequestParams {
        contract: ctx.contract_id(),
        function: Hname::new("increment_repeat_many"),
        params: None,
        transfer: None,
        delay: 0,
    });
}

fn increment_when_must_increment(ctx: &ScCallContext) {
    ctx.log("increment_when_must_increment called");
    unsafe {
        if !LOCAL_STATE_MUST_INCREMENT {
            return;
        }
    }
    let counter = ctx.state().get_int(KEY_COUNTER);
    counter.set_value(counter.value() + 1);
}

fn increment_local_state_internal_call(ctx: &ScCallContext) {
    increment_when_must_increment(ctx);
    unsafe {
        LOCAL_STATE_MUST_INCREMENT = true;
    }
    increment_when_must_increment(ctx);
    increment_when_must_increment(ctx);
    // counter ends up as 2
}

fn increment_local_state_sandbox_call(ctx: &ScCallContext) {
    ctx.call(ctx.contract_id().hname(), Hname::new("increment_when_must_increment"), None, None);
    unsafe {
        LOCAL_STATE_MUST_INCREMENT = true;
    }
    ctx.call(ctx.contract_id().hname(), Hname::new("increment_when_must_increment"), None, None);
    ctx.call(ctx.contract_id().hname(), Hname::new("increment_when_must_increment"), None, None);
    // counter ends up as 0
}

fn increment_local_state_post(ctx: &ScCallContext) {
    let request = PostRequestParams {
        contract: ctx.contract_id(),
        function: Hname::new("increment_when_must_increment"),
        params: None,
        transfer: None,
        delay: 0,
    };
    ctx.post(&request);
    unsafe {
        LOCAL_STATE_MUST_INCREMENT = true;
    }
    ctx.post(&request);
    ctx.post(&request);
    // counter ends up as 0
}

fn test(_sc: &ScCallContext) {
    let key_id = get_key_id_from_string("timestamp");
    set_bytes(1, key_id, TYPE_INT, &123456789_i64.to_le_bytes());
    let bytes = get_bytes(1, key_id, TYPE_INT);
    let timestamp = i64::from_le_bytes(bytes.try_into().unwrap());
    set_bytes(1, key_id, TYPE_INT, &timestamp.to_le_bytes());
    let key_id2 = get_key_id_from_string("string");
    set_bytes(1, key_id2, TYPE_STRING, "Test".as_bytes());
    let s1 = get_bytes(1, key_id2, TYPE_STRING);
    set_bytes(1, key_id2, TYPE_STRING, "Bleep".as_bytes());
    let s2 = get_bytes(1, key_id2, TYPE_STRING);
    set_bytes(1, key_id2, TYPE_STRING, "Klunky".as_bytes());
    let s3 = get_bytes(1, key_id2, TYPE_STRING);
    set_bytes(1, key_id2, TYPE_STRING, &s1);
    set_bytes(1, key_id2, TYPE_STRING, &s2);
    set_bytes(1, key_id2, TYPE_STRING, &s3);
}

fn results_test(ctx: &ScCallContext) {
    test_map(ctx.results());
    check_map(ctx.results().immutable());
    //ctx.call(ctx.contract_id().hname(), Hname::new("results_check"), None, None);
}

fn state_test(ctx: &ScCallContext) {
    test_map(ctx.state());
    ctx.call(ctx.contract_id().hname(), Hname::new("state_check"), None, None);
}

fn results_check(ctx: &ScViewContext) {
    check_map(ctx.results().immutable());
}

fn state_check(ctx: &ScViewContext) {
    check_map(ctx.state());
}

fn test_map(kvstore: ScMutableMap) {
    let int1 = kvstore.get_int("int1");
    check(int1.value() == 0);
    int1.set_value(1);

    let string1 = kvstore.get_string("string1");
    check(string1.value() == "");
    string1.set_value("a");

    let ia1 = kvstore.get_int_array("ia1");
    let int2 = ia1.get_int(0);
    check(int2.value() == 0);
    int2.set_value(2);
    let int3 = ia1.get_int(1);
    check(int3.value() == 0);
    int3.set_value(3);

    let sa1 = kvstore.get_string_array("sa1");
    let string2 = sa1.get_string(0);
    check(string2.value() == "");
    string2.set_value("bc");
    let string3 = sa1.get_string(1);
    check(string3.value() == "");
    string3.set_value("def");
}

fn check_map(kvstore: ScImmutableMap) {
    let int1 = kvstore.get_int("int1");
    check(int1.value() == 1);

    let string1 = kvstore.get_string("string1");
    check(string1.value() == "a");

    let ia1 = kvstore.get_int_array("ia1");
    let int2 = ia1.get_int(0);
    check(int2.value() == 2);
    let int3 = ia1.get_int(1);
    check(int3.value() == 3);

    let sa1 = kvstore.get_string_array("sa1");
    let string2 = sa1.get_string(0);
    check(string2.value() == "bc");
    let string3 = sa1.get_string(1);
    check(string3.value() == "def");
}

// fn check_map_rev(kvstore: ScImmutableMap) {
//     let sa1 = kvstore.get_string_array("sa1");
//     let string3 = sa1.get_string(1);
//     check(string3.value() == "def");
//     let string2 = sa1.get_string(0);
//     check(string2.value() == "bc");
//
//     let ia1 = kvstore.get_int_array("ia1");
//     let int3 = ia1.get_int(1);
//     check(int3.value() == 3);
//     let int2 = ia1.get_int(0);
//     check(int2.value() == 2);
//
//     let string1 = kvstore.get_string("string1");
//     check(string1.value() == "a");
//
//     let int1 = kvstore.get_int("int1");
//     check(int1.value() == 1);
// }

fn check(condition: bool) {
    if !condition {
        panic!("Check failed!")
    }
}
