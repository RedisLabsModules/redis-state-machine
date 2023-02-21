use redis_module::{Context, RedisError, RedisResult, RedisString, NextArg, RedisValue, key::{RedisKeyWritable}};

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
    match val {
        Err(e) => {
            return Err(e);
        }
        Ok(v) => {
            if v.is_none() {
                return Ok(RedisValue::Null);
            } else {
                return kk.delete();
            }
        }
    }
}