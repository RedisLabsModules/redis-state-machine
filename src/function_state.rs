use redis_module::{Context, RedisError, RedisString, RedisResult, RedisValue, key::RedisKey};
use crate::REDIS_SM_TYPE;
use crate::types::StateMachine;

pub(crate) fn state(ctx: &Context, args: Vec<RedisString>) -> RedisResult {

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

// pub(crate) fn states(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
// }