use redis_module::{Context, RedisError, RedisString, RedisResult, RedisValue, key::RedisKey, REDIS_OK};
use crate::REDIS_SM_TYPE;
use crate::types::StateMachine;

pub(crate) fn current_state(ctx: &Context, args: Vec<RedisString>) -> RedisResult {

    let args = args.into_boxed_slice();
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }
    let key = &args[1];

    let rkey = RedisKey::open(ctx.ctx, &key);
    let val = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE);

    match val {
        Err(e) => {
            return Err(e);
        }
        Ok(v) => {
            if v.is_none() {
                return Ok(RedisValue::Null);
            } else {
                let sm = val.unwrap();
                let vvv = sm.unwrap();
                return Ok(RedisValue::SimpleString(vvv.current().to_string()));
            }
        }
    }

}

pub(crate) fn states(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_boxed_slice();
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }
    let key = &args[1];

    let rkey = RedisKey::open(ctx.ctx, &key);
    let val = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE);
    match val {
       Err(e) => {
            return Err(e);
        }
        Ok(v) => {
            if v.is_none() {
                return Ok(RedisValue::Null);
            } else {
                let sm= v.unwrap();
                let mut keys : Vec<RedisValue> = Vec::new();
                for x in sm.map().keys() {
                    keys.push(RedisValue::SimpleString(x.to_string()));
                }
                return Ok(RedisValue::Array(keys));
            }
        }
    }

}