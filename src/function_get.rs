use crate::types::{StateMachine};
use crate::REDIS_SM_TYPE;
use redis_module::{
    key::RedisKey, Context, NextArg, RedisError, RedisResult, RedisString, RedisValue,
};

pub(crate) fn get(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;

    let rkey = RedisKey::open(ctx.ctx, &key);

    let v = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if v.is_none() {
        Ok(RedisValue::Null)
    } else {
        let rval = serde_json::to_string(&v)?;
        Ok(RedisValue::BulkString(rval))
    }
}
