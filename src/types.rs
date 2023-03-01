use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::{collections::HashMap};
use redis_module::{RedisString};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StateMachine {
    current: Option<String>,
    map: HashMap<String, Vec<String>>,
    initial: String,

    // TODO store some way for this to never change
}

pub(crate) fn new_from_string(c: RedisString) -> Result<StateMachine, Error> {
    let rval = serde_json::from_str(&c.to_string());
    return rval;
}

pub(crate) fn new() -> StateMachine {
    let m  : HashMap<String, Vec<String>> = HashMap::new();
    let sm = StateMachine{
        initial: String::from(""),
        current: Option::None,
        map: m
    };
    return sm;
}

// pub(crate) fn new_from_key(rkey: RedisKey) -> StateMachine {
//     let value = rkey.get_value::<StateMachine>(&REDIS_SM_TYPE);
//     if value.is_err() {
//         return new();
//     }
//     let res = value.unwrap();
//     let sm =  res.unwrap();
//     return sm.into();
// }

impl StateMachine {

    pub(crate) fn set_current(&mut self, c: String) {
        self.current = Option::from(c);
    }

    pub(crate) fn current(&self) -> &str {
        self.current.as_ref().map_or("", |p| p)
    }

    pub(crate) fn initial(&self) -> &str {
        &self.initial
    }

    pub(crate) const fn map(&self) -> &HashMap<String, Vec<String>> {
        &self.map
    }

}
