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

    // pub(crate) fn get_current(&self) -> Option<String> {
    //     return self.current
    // }

    pub(crate) fn set_initial(&mut self, c: String) {
        self.initial = c;
    }

    // pub(crate) fn get_initial(&self) -> String {
    //     return self.initial
    // }

}
