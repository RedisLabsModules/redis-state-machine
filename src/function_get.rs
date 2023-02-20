use redis_module::{Context, RedisError, RedisString, RedisResult, RedisValue, REDIS_OK, key::RedisKey};
use redis_module::key::{verify_type};
use crate::REDIS_SM_TYPE;
use crate::types::StateMachine;
use serde_json::Value;

pub(crate) fn get(ctx: &Context, args: Vec<RedisString>) -> RedisResult {

    let args = args.into_boxed_slice();
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }
    let key = &args[1];

    let rkey = RedisKey::open(ctx.ctx, &key);
    let val =rkey.get_value::<StateMachine>(&REDIS_SM_TYPE);

    // return Ok(RedisValue::Null);

    // // return RedisValue::SimpleString(())
    return REDIS_OK;
}