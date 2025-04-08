pub struct State {
    kvp: HashMap<String, String>,
}

impl State {
    fn new() -> Self {
        State {
            kvp: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.kvp.set(key, value)
    }

    fn get(&self, key: String) -> Option<String> {
        self.get(key)
    }
}
