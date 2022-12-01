use redis_module::{Context, RedisError, RedisResult, RedisString, NextArg, RedisValue, REDIS_OK, key::{RedisKey, RedisKeyWritable}};
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
    return kk.delete();
}
