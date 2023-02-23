use crate::rdb::REDIS_SM_TYPE;
use crate::types::StateMachine;
use redis_module::{
    key::RedisKeyWritable, Context, RedisError, RedisResult, RedisString, RedisValue, REDIS_OK,
};

// Load the state machine from a json string
pub(crate) fn set(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_boxed_slice();
    if args.len() > 4 || args.len() < 3 {
        return Err(RedisError::WrongArity);
    }

    let key = &args[1];
    let val = &args[2];
    let mut current = &RedisString::create(std::ptr::null_mut(), "");
    if args.len() == 4 {
        current = &args[3];
    }

    let mut rval: StateMachine = serde_json::from_str(&val.to_string())?;

    if !current.is_empty() {
        rval.set_current(current.to_string());
    }
    if rval.current().is_empty() {
        rval.set_current(rval.initial().to_string());
    }

    let rkey = RedisKeyWritable::open(ctx.ctx, key);
    rkey.set_value(&REDIS_SM_TYPE, rval)?;
    REDIS_OK
}

// Reset the state machine to the initial state
pub(crate) fn reset(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_boxed_slice();
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }

    let key = &args[1];
    let rkey = RedisKeyWritable::open(ctx.ctx, key);
    let v = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if let Some(rval) = v {
        rval.set_current(rval.initial().to_string());
        rkey.set_value(&REDIS_SM_TYPE, rval)?;
        REDIS_OK
    } else {
        Ok(RedisValue::Null)
    }
}

// Force set the named state machine to a value
pub(crate) fn force_set(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_boxed_slice();
    if args.len() != 3 {
        return Err(RedisError::WrongArity);
    }

    let key = &args[1];
    let state = &args[2];

    let rkey = RedisKeyWritable::open(ctx.ctx, key);
    let v = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if let Some(rval) = v {
        rval.set_current(state.to_string());
        rkey.set_value(&REDIS_SM_TYPE, rval)?;
        REDIS_OK
    } else {
        Ok(RedisValue::Null)
    }
}
