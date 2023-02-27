use crate::types::StateMachine;
use crate::REDIS_SM_TYPE;
use redis_module::{key::RedisKey, Context, RedisError, RedisResult, RedisString, RedisValue};

pub(crate) fn current_state(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_boxed_slice();
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }
    let key = &args[1];

    let rkey = RedisKey::open(ctx.ctx, key);
    let v = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if let Some(vvv) = v {
        Ok(RedisValue::SimpleString(vvv.current().to_string()))
    } else {
        Ok(RedisValue::Null)
    }
}

pub(crate) fn states(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_boxed_slice();
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }
    let key = &args[1];

    let rkey = RedisKey::open(ctx.ctx, key);
    let v = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if let Some(sm) = v {
        let mut keys: Vec<RedisValue> = Vec::new();
        for x in sm.map().keys() {
            keys.push(RedisValue::SimpleString(x.to_string()));
        }
        Ok(RedisValue::Array(keys))
    } else {
        Ok(RedisValue::Null)
    }
}
