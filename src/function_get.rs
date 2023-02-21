use crate::types::StateMachine;
use crate::REDIS_SM_TYPE;
use redis_module::{key::RedisKey, Context, RedisError, RedisResult, RedisString, RedisValue};

pub(crate) fn get(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let args = args.into_boxed_slice();
    if args.len() < 2 {
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
                let rval = serde_json::to_string(&sm)?;
                return Ok(RedisValue::BulkString(rval));
            }
        }
    }
}
