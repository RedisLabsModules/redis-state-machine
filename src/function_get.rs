use crate::types::StateMachine;
use crate::REDIS_SM_TYPE;
use redis_module::{key::RedisKey, Context, RedisError, RedisResult, RedisString, RedisValue};

pub(crate) fn get(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_boxed_slice();
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }
    let key = &args[1];

    let rkey = RedisKey::open(ctx.ctx, key);
    let v = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if v.is_none() {
        Ok(RedisValue::Null)
    } else {
        let rval = serde_json::to_string(&v)?;
        Ok(RedisValue::BulkString(rval))
    }
}
