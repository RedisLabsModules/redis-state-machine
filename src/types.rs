use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StateMachine{

    current: Option<String>,
    map: HashMap<String, Vec<String>>,
    initial: String,
}

impl StateMachine {

    pub(crate) fn set_current(&mut self, c: String) {
        self.current = Option::from(c);
    }

    pub(crate) fn current(&self) -> &str {
        let v = self.current.as_ref().unwrap();
        return v;
    }

    pub(crate) fn initial(&self) -> &str {
        return &self.initial;
    }

}
