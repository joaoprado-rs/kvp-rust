use std::collections::HashMap;

pub struct State {
    pub kvp: HashMap<String, String>,
}

impl State {
    pub fn new() -> Self {
        State {
            kvp: HashMap::new(),
        }
    }
}
