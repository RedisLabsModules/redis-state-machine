use redis_module::{Context, RedisError, RedisResult, RedisString, NextArg, RedisValue, REDIS_OK, key::{RedisKey, RedisKeyWritable}};
use serde_json::{Result, Value};
use crate::rdb::REDIS_SM_TYPE;
use crate::types::StateMachine;
use try_catch::catch;


// Load the state machine from a json string
pub(crate) fn load(ctx: &Context, args: Vec<RedisString>) -> RedisResult {

    let args = args.into_boxed_slice();
    if args.len() < 3 {
        return Err(RedisError::WrongArity);
    }

    let key = &args[1];
    let val = &args[2];
    let mut current= &RedisString::create(std::ptr::null_mut(), "");
    if args.len() == 4 {
        current= &args[3];
    } else {
    }

    let mut rval: StateMachine = serde_json::from_str(&val.to_string())?;

    if ! current.is_empty() {
        rval.set_current(current.to_string());
    }

    let rkey = RedisKeyWritable::open(ctx.ctx, &key);
    rkey.set_value(&REDIS_SM_TYPE,rval);
    return REDIS_OK;
}