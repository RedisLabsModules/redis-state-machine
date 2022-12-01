use redis_module::{Context, RedisError, RedisResult, RedisString, NextArg, RedisValue, REDIS_OK, key::{RedisKey, RedisKeyWritable}};

// Load the state machine from a json string
pub(crate) fn load(_: &Context, args: Vec<RedisString>) -> RedisResult {

    let mut args = args.into_boxed_slice();
    // let mut args = args.into_iter();
    if args.len() != 3 {
        return Err(RedisError::WrongArity);
    }
    let key = &args[1];
    let val = &args[2];

    return REDIS_OK;
}