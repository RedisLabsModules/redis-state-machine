use crate::types::{new, StateMachine};
use crate::REDIS_SM_TYPE;
use redis_module::{
    key::RedisKey, Context, NextArg, RedisError, RedisResult, RedisString, RedisValue,
};

pub(crate) fn get(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let fieldarg = args.next_arg();

    let rkey = RedisKey::open(ctx.ctx, &key);

    let v = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if v.is_none() {
        return Ok(RedisValue::Null);
    }

    let rval = serde_json::to_string(&v)?;
    if fieldarg.is_err() {
        Ok(RedisValue::BulkString(rval))
    } else if fieldarg.unwrap().to_string().to_uppercase() == "REASON" {
        let sm = v.unwrap();
        Ok(RedisValue::SimpleString(sm.reason().to_string()))
    } else {
        Ok(RedisValue::Null)
    }
}

pub(crate) fn template(_ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }
    let n = new();
    let rval = serde_json::to_string(&n)?;
    Ok(RedisValue::BulkString(rval))
}
