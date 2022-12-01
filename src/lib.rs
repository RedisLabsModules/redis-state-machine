use redis_module::native_types::RedisType;

#[macro_use]

extern crate redis_module;

pub const REDIS_SM_TYPE_VERSION: i32 = 1;
pub const MODULE_NAME: &str = "RedisStateMachine";
pub const MODULE_TYPE: &str = "RedisStateMachine-Store";

mod function_delete;
mod function_load;

redis_module! {
    name: "redisstate",
    version: 1,
    data_types: [],
    commands: [
        ["SM.LOAD", function_load::load, "write deny-oom", 1, 1, 1],
        ["SM.DEL", function_delete::delete, "write", 1, 1, 1],
    //     ["SM.RESET", commands::load, "write", 1, 1, 1],
    //     ["SM.GET", commands::get, "readonly", 1, 1, 1],
    //     ["SM.CURRENT", commands::state, "readonly", 1, 1, 1],
    ],
}