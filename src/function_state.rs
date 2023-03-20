use crate::types::StateMachine;
use crate::REDIS_SM_TYPE;
use redis_module::{
    key::{RedisKey, RedisKeyWritable},
    Context, NextArg, RedisError, RedisResult, RedisString, RedisValue, REDIS_OK,
};

pub(crate) fn state(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 2 || args.len() > 3 {
        return Err(RedisError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);

    let key = args.next_arg()?;
    let list = args.next_arg();

    let rkey = RedisKey::open(ctx.ctx, &key);
    let value = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if value.is_none() {
        return Ok(RedisValue::Null);
    }

    let sm = value.unwrap();
    let mut keys: Vec<RedisValue> = Vec::new();
    if list.is_ok() {
        for x in sm.map().keys() {
            keys.push(RedisValue::SimpleString(x.to_string()));
        }
    } else {
        keys.push(RedisValue::SimpleString(sm.current().to_string()));
    }
    Ok(RedisValue::Array(keys))
}

pub(crate) fn mutate(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 3 || args.len() > 6 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let target = args.next_arg()?;

    let maybe_reason = args.next_arg();
    let maybe_options = args.next_arg();

    let rkey = RedisKeyWritable::open(ctx.ctx, &key);
    let value = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if value.is_none() {
        return Ok(RedisValue::Null);
    }

    let sm = value.unwrap();
    if !sm.is_valid_state(target.to_string()) {
        return Err(RedisError::String("Invaild state transition".to_string()));
    }

    let res = sm.can_transition(target.to_string());

    if let Ok(options) = maybe_options {
        if options.to_string().to_uppercase() != "FORCE" {
            return Err(RedisError::String("Invaild command option".to_string()));
        }
    } else if !res {
        return Err(RedisError::String("Invaild state transition".to_string()));
    }

    sm.set_current(target.to_string());
    if let Ok(value) = maybe_reason {
        sm.set_reason(value.to_string());
    }
    REDIS_OK
}
