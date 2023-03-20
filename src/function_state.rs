use crate::types::StateMachine;
use crate::REDIS_SM_TYPE;
use redis_module::{
    key::{RedisKey, RedisKeyWritable},
    Context, NextArg, RedisError, RedisResult, RedisString, RedisValue, REDIS_OK,
};

pub(crate) fn state(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 2 || args.len() > 3{
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

    //     return value.map_or_else(
    //         || Ok(RedisValue::Null),
    //         |sm| Ok(RedisValue::SimpleString(sm.current().to_string())),
    //     );
    // }

    let sm = value.unwrap();
    let mut keys: Vec<RedisValue> = Vec::new();
    if ! list.is_err() {
        for x in sm.map().keys() {
            keys.push(RedisValue::SimpleString(x.to_string()));
        }
    } else if list.unwrap().to_string().to_uppercase() == "LIST" {
        keys.push(RedisValue::SimpleString(sm.current().to_string()));
    } else {
        return Ok(RedisValue::Null);
    }
    Ok(RedisValue::Array(keys))

    // if let Some(sm) = value {
    //     let mut keys: Vec<RedisValue> = Vec::new();
    //     for x in sm.map().keys() {
    //         keys.push(RedisValue::SimpleString(x.to_string()));
    //     }
    //     Ok(RedisValue::Array(keys))
    // } else {
    //     Ok(RedisValue::Null)
    // }

}

// pub(crate) fn states(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
//     if args.len() != 2 {
//         return Err(RedisError::WrongArity);
//     }
//     let mut args = args.into_iter().skip(1);
//     let key = args.next_arg()?;

//     let rkey = RedisKey::open(ctx.ctx, &key);
//     let value = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

//     if let Some(sm) = value {
//         let mut keys: Vec<RedisValue> = Vec::new();
//         for x in sm.map().keys() {
//             keys.push(RedisValue::SimpleString(x.to_string()));
//         }
//         Ok(RedisValue::Array(keys))
//     } else {
//         Ok(RedisValue::Null)
//     }
// }

pub(crate) fn mutate(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 3 || args.len() > 5 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let target = args.next_arg()?;
    let force = args.next_arg();

    let rkey = RedisKeyWritable::open(ctx.ctx, &key);
    let value = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE)?;

    if value.is_none() {
        return Ok(RedisValue::Null);
    }

    let sm = value.unwrap();
    if force.is_err() {
        let res = sm.check_transition(target.to_string());
        if !res {
            return Ok(RedisValue::Null);
        }
        sm.set_current(target.to_string());
    } else if force.unwrap().to_string().to_uppercase() == "FORCE" {
        sm.set_current(target.to_string());
    } else {
        return Ok(RedisValue::Null);
    }

    REDIS_OK
}
