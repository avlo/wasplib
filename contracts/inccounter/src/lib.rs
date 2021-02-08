// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
//////// DO NOT CHANGE THIS FILE! ////////
// Change the json schema instead

use inccounter::*;
use schema::*;
use wasmlib::*;

mod inccounter;
mod schema;

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_call(FUNC_CALL_INCREMENT, func_call_increment_thunk);
    exports.add_call(FUNC_CALL_INCREMENT_RECURSE5X, func_call_increment_recurse5x_thunk);
    exports.add_call(FUNC_INCREMENT, func_increment_thunk);
    exports.add_call(FUNC_INIT, func_init_thunk);
    exports.add_call(FUNC_LOCAL_STATE_INTERNAL_CALL, func_local_state_internal_call_thunk);
    exports.add_call(FUNC_LOCAL_STATE_POST, func_local_state_post_thunk);
    exports.add_call(FUNC_LOCAL_STATE_SANDBOX_CALL, func_local_state_sandbox_call_thunk);
    exports.add_call(FUNC_POST_INCREMENT, func_post_increment_thunk);
    exports.add_call(FUNC_REPEAT_MANY, func_repeat_many_thunk);
    exports.add_call(FUNC_RESULTS_TEST, func_results_test_thunk);
    exports.add_call(FUNC_STATE_TEST, func_state_test_thunk);
    exports.add_call(FUNC_WHEN_MUST_INCREMENT, func_when_must_increment_thunk);
    exports.add_view(VIEW_GET_COUNTER, view_get_counter_thunk);
    exports.add_view(VIEW_RESULTS_CHECK, view_results_check_thunk);
    exports.add_view(VIEW_STATE_CHECK, view_state_check_thunk);
}

pub struct FuncCallIncrementParams {
}

fn func_call_increment_thunk(ctx: &ScCallContext) {
    let params = FuncCallIncrementParams {
    };
    func_call_increment(ctx, &params);
}

pub struct FuncCallIncrementRecurse5xParams {
}

fn func_call_increment_recurse5x_thunk(ctx: &ScCallContext) {
    let params = FuncCallIncrementRecurse5xParams {
    };
    func_call_increment_recurse5x(ctx, &params);
}

pub struct FuncIncrementParams {
}

fn func_increment_thunk(ctx: &ScCallContext) {
    let params = FuncIncrementParams {
    };
    func_increment(ctx, &params);
}

pub struct FuncInitParams {
    pub counter: ScImmutableInt, // value to initialize state counter with
}

fn func_init_thunk(ctx: &ScCallContext) {
    let p = ctx.params();
    let params = FuncInitParams {
        counter: p.get_int(PARAM_COUNTER),
    };
    func_init(ctx, &params);
}

pub struct FuncLocalStateInternalCallParams {
}

fn func_local_state_internal_call_thunk(ctx: &ScCallContext) {
    let params = FuncLocalStateInternalCallParams {
    };
    func_local_state_internal_call(ctx, &params);
}

pub struct FuncLocalStatePostParams {
}

fn func_local_state_post_thunk(ctx: &ScCallContext) {
    let params = FuncLocalStatePostParams {
    };
    func_local_state_post(ctx, &params);
}

pub struct FuncLocalStateSandboxCallParams {
}

fn func_local_state_sandbox_call_thunk(ctx: &ScCallContext) {
    let params = FuncLocalStateSandboxCallParams {
    };
    func_local_state_sandbox_call(ctx, &params);
}

pub struct FuncPostIncrementParams {
}

fn func_post_increment_thunk(ctx: &ScCallContext) {
    let params = FuncPostIncrementParams {
    };
    func_post_increment(ctx, &params);
}

pub struct FuncRepeatManyParams {
    pub num_repeats: ScImmutableInt, // number of times to recursively call myself
}

fn func_repeat_many_thunk(ctx: &ScCallContext) {
    let p = ctx.params();
    let params = FuncRepeatManyParams {
        num_repeats: p.get_int(PARAM_NUM_REPEATS),
    };
    func_repeat_many(ctx, &params);
}

pub struct FuncResultsTestParams {
}

fn func_results_test_thunk(ctx: &ScCallContext) {
    let params = FuncResultsTestParams {
    };
    func_results_test(ctx, &params);
}

pub struct FuncStateTestParams {
}

fn func_state_test_thunk(ctx: &ScCallContext) {
    let params = FuncStateTestParams {
    };
    func_state_test(ctx, &params);
}

pub struct FuncWhenMustIncrementParams {
}

fn func_when_must_increment_thunk(ctx: &ScCallContext) {
    let params = FuncWhenMustIncrementParams {
    };
    func_when_must_increment(ctx, &params);
}

pub struct ViewGetCounterParams {
}

fn view_get_counter_thunk(ctx: &ScViewContext) {
    let params = ViewGetCounterParams {
    };
    view_get_counter(ctx, &params);
}

pub struct ViewResultsCheckParams {
}

fn view_results_check_thunk(ctx: &ScViewContext) {
    let params = ViewResultsCheckParams {
    };
    view_results_check(ctx, &params);
}

pub struct ViewStateCheckParams {
}

fn view_state_check_thunk(ctx: &ScViewContext) {
    let params = ViewStateCheckParams {
    };
    view_state_check(ctx, &params);
}
