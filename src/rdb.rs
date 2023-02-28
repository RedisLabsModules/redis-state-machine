use redis_module::{native_types::RedisType, RedisModuleTypeMethods, REDISMODULE_AUX_BEFORE_RDB};

use redis_module::raw;
use std::mem;
use std::os::raw::c_void;

use crate::types::StateMachine;

pub(crate) static REDIS_SM_VERSION: i32 = 1;
pub(crate) static REDIS_SM_TYPE: RedisType = RedisType::new(
    "StateType",
    REDIS_SM_VERSION,
    RedisModuleTypeMethods {
        version: redis_module::TYPE_METHOD_VERSION,
        rdb_load: None,
        rdb_save: None, //Some(rdb_save),
        aof_rewrite: None,
        free: Some(free),
        mem_usage: Some(mem_usage),
        digest: None,
        aux_load: None,
        aux_save: None,
        aux_save_triggers: REDISMODULE_AUX_BEFORE_RDB as i32,
        free_effort: None,
        unlink: None,
        copy: Some(copy),
        defrag: None,
    },
);

pub unsafe extern "C" fn mem_usage(value: *const c_void) -> usize {
    let sm = unsafe { &*(value as *mut StateMachine) };
    mem::size_of_val(sm)
}

#[allow(non_snake_case, unused)]
pub unsafe extern "C" fn free(value: *mut c_void) {
    let sm = unsafe { value as *mut StateMachine };
    Box::from_raw(sm);
}

#[allow(non_snake_case, unused)]
pub unsafe extern "C" fn copy(
    fromkey: *mut raw::RedisModuleString,
    tokey: *mut raw::RedisModuleString,
    value: *const c_void,
) -> *mut c_void {
    let sm = unsafe { &*(value as *mut StateMachine) };
    Box::into_raw(Box::new(sm)).cast::<c_void>()
}
