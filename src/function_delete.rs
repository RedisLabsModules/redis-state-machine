use redis_module::{
    key::RedisKeyWritable, Context, NextArg, RedisError, RedisResult, RedisString, RedisValue,
};

use crate::rdb::REDIS_SM_TYPE;
use crate::types::StateMachine;

// Delete a state machine
pub(crate) fn delete(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    if args.len() > 0 {
        return Err(RedisError::WrongArity);
    }
    if key.is_empty() {
        return Err(RedisError::Str("Empty key specified"));
    }

    let kk = RedisKeyWritable::open(ctx.ctx, &key);

    let v = kk.get_value::<StateMachine>(&REDIS_SM_TYPE)?;
    if v.is_none() {
        Ok(RedisValue::Null)
    } else {
        kk.delete()
    }
}
