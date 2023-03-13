use redis_module::RedisString;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct StateMachine {
    current: String,
    map: HashMap<String, Vec<String>>,
    initial: String,
    // TODO store some way for this to never change
}

pub(crate) fn new_from_redisstring(c: RedisString) -> Result<StateMachine, Error> {
    serde_json::from_str(&c.to_string())
}

pub(crate) fn new() -> StateMachine {
    let m: HashMap<String, Vec<String>> = HashMap::new();
    StateMachine {
        initial: String::from(""),
        current: String::from(""),
        map: m,
    }
}

impl StateMachine {
    pub(crate) fn set_current(&mut self, c: String) {
        self.current = c;
    }

    pub(crate) fn set_current_from_redisstring(&mut self, c: RedisString) {
        self.current = c.to_string();
    }

    pub(crate) fn current(&self) -> &str {
        &self.current
    }

    pub(crate) fn initial(&self) -> &str {
        &self.initial
    }

    pub(crate) const fn map(&self) -> &HashMap<String, Vec<String>> {
        &self.map
    }
}
