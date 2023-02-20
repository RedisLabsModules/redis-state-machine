use redis_module::{
    RedisModuleTypeMethods, native_types::RedisType, REDISMODULE_AUX_BEFORE_RDB,

};


pub(crate) static REDIS_SM_VERSION: i32 = 1;
pub(crate) static REDIS_SM_TYPE: RedisType = RedisType::new(
    "StateType",
    REDIS_SM_VERSION,
    RedisModuleTypeMethods{
        version: redis_module::TYPE_METHOD_VERSION,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        free: None,
        mem_usage: None,
        digest: None,
        aux_load: None,
        aux_save: None,
        aux_save_triggers: REDISMODULE_AUX_BEFORE_RDB as i32,
        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,
    },
);