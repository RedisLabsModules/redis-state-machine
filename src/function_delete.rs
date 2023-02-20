use redis_module::{Context, RedisError, RedisResult, RedisString, NextArg, RedisValue, REDIS_OK, key::{RedisKey, RedisKeyWritable}};

use crate::rdb::REDIS_SM_TYPE;
use crate::types::StateMachine;

// Delete a state machine
pub(crate) fn delete(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    if args.len() > 0 {
        return Err(RedisError::WrongArity);
    }
    if key.is_empty()  {
        return Err(RedisError::Str("Empty key specified"));
    }

    let kk = RedisKeyWritable::open(ctx.ctx, &key);

    let val = kk.get_value::<StateMachine>(&REDIS_SM_TYPE);
    println!("{:#?}", val);
    // TODO only delete if this is from the right type
    return kk.delete();
}