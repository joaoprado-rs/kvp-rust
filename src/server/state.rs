use std::collections::HashMap;

pub struct State {
    kvp: HashMap<String, String>,
}

impl State {
    pub fn new() -> Self {
        State {
            kvp: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.kvp.insert(key, value)
    }

    fn get(&self, key: &str) -> Option<String> {
        self.kvp.get(key).cloned()
    }
}
