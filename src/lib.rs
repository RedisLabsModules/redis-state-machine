use rdb::REDIS_SM_TYPE;

#[macro_use]
extern crate redis_module;

pub const REDIS_SM_TYPE_VERSION: i32 = 1;
pub const MODULE_NAME: &str = "RedisStateMachine";
pub const MODULE_TYPE: &str = "RedisStateMachine";

mod function_delete;
mod function_get;
mod function_set;
mod function_state;
mod rdb;
mod types;

redis_module! {
    name: "redisstate",
    version: 1,
    data_types: [REDIS_SM_TYPE],
    commands: [
        ["SM.GET", function_get::get, "readonly", 0, 0, 0],
        ["SM.SET", function_set::set, "write deny-oom", 1, 1, 1],
        ["SM.CURRENT", function_state::current_state, "readonly", 0, 0, 0],
        ["SM.STATES", function_state::states, "readonly", 0, 0, 0],
        ["SM.DEL", function_delete::delete, "write", 1, 1, 1],
        ["SM.CREATE", function_set::create, "write deny-oom", 1, 1, 1],
        ["SM.TEMPLATE", function_get::template, "readonly", 0, 0, 0],
        ["SM.FORCE", function_set::force_set, "write deny-oom", 1, 1, 1],
        ["SM.RESET", function_set::reset, "write deny-oom", 1, 1, 1],

        // ["SM.GO", function_state::go "write deny-oom", 1, 1, 1],
    ],
}
