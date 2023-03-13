use redis_module::{native_types::RedisType, RedisModuleTypeMethods, REDISMODULE_AUX_BEFORE_RDB};

use redis_module::raw;
use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;

use crate::types::{new_from_redisstring, StateMachine};

pub(crate) static REDIS_SM_VERSION: i32 = 1;
pub(crate) static REDIS_SM_TYPE: RedisType = RedisType::new(
    "StateType",
    REDIS_SM_VERSION,
    RedisModuleTypeMethods {
        version: redis_module::TYPE_METHOD_VERSION,
        rdb_load: Some(rdb_load),
        rdb_save: Some(rdb_save),
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

unsafe extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
    let v = &*value.cast::<StateMachine>();
    raw::save_string(rdb, &serde_json::to_string(&v).unwrap());
}

unsafe extern "C" fn rdb_load(rdb: *mut raw::RedisModuleIO, _encver: c_int) -> *mut c_void {
    let v = raw::load_string(rdb);
    if v.is_err() {
        return null_mut();
    }
    let f = v.unwrap();
    let sm = new_from_redisstring(f);
    if sm.is_err() {
        return null_mut();
    }
    let ff = sm.unwrap();
    let bb = Box::new(ff);
    let rawbox = Box::into_raw(bb);
    rawbox as *mut c_void
}

unsafe extern "C" fn mem_usage(value: *const c_void) -> usize {
    let sm = unsafe { &*(value as *mut StateMachine) };
    mem::size_of_val(sm)
}

#[allow(unused)]
unsafe extern "C" fn free(value: *mut c_void) {
    if value.is_null() {
        return;
    }
    let sm =  value as *mut StateMachine ;
    Box::from_raw(sm);
}

#[allow(non_snake_case, unused)]
unsafe extern "C" fn copy(
    fromkey: *mut raw::RedisModuleString,
    tokey: *mut raw::RedisModuleString,
    value: *const c_void,
) -> *mut c_void {
    let sm =  &*(value as *mut StateMachine) ;
    let newSm = sm.clone();
    Box::into_raw(Box::new(newSm)).cast::<c_void>()
}
