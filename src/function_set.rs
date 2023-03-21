use crate::rdb::REDIS_SM_TYPE;
use crate::types::{new, new_from_redisstring, StateMachine};
use redis_module::{
    key::RedisKeyWritable, Context, NextArg, RedisResult, RedisString, RedisValue,
    REDIS_OK,
};

// Load the state machine from a json string
pub(crate) fn set(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let val = args.next_arg()?;

    // If the current state is not specified, then use the initial state
    let current = args
        .next_arg()
        .unwrap_or_else(|_| RedisString::create(std::ptr::null_mut(), ""));

    let mut rval: StateMachine = new_from_redisstring(val)?;

    if !current.is_empty() {
        rval.set_current(current.to_string());
    }
    if rval.current().is_empty() {
        rval.set_current(rval.initial().to_string());
    }

    let rkey = RedisKeyWritable::open(ctx.ctx, &key);
    rkey.set_value(&REDIS_SM_TYPE, rval)?;
    REDIS_OK
}

pub(crate) fn create(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;

    let sm = new();
    let rkey = RedisKeyWritable::open(ctx.ctx, &key);
    rkey.set_value(&REDIS_SM_TYPE, sm)?;
    REDIS_OK
}

// Reset the state machine to the initial state
pub(crate) fn reset(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;

    let rkey = RedisKeyWritable::open(ctx.ctx, &key);
    let v = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if let Some(rval) = v {
        rval.set_current(rval.initial().to_string());
        REDIS_OK
    } else {
        Ok(RedisValue::Null)
    }
}
