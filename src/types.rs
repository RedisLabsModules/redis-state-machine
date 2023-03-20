use redis_module::RedisString;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct StateMachine {
    current: String,
    map: HashMap<String, Vec<String>>,
    initial: String,
    reason: String,
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
        reason: String::from(""),
    }
}

impl StateMachine {
    pub(crate) fn set_current(&mut self, c: String) {
        self.current = c;
    }

    pub(crate) fn set_reason(&mut self, c: String) {
        self.reason = c;
    }

    pub(crate) fn can_transition(&self, target: String) -> bool {
        let current = String::from(self.current());
        let mapval = self.map.get(&current);
        if mapval.is_none() {
            return false;
        }
        let v = mapval.unwrap();
        v.contains(&target)
    }

    pub(crate) fn is_valid_state(&self, target: String) -> bool {
        self.map.contains_key(&target)
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

    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }
}
