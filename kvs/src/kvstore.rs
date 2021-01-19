use std::collections::HashMap;

/// The structure of kvstore
#[derive(Default)]
pub struct KvStore {
    kv: HashMap<String, String>,
}

impl KvStore {
    /// Create a new KvStore
    pub fn new() -> KvStore {
        KvStore { kv: HashMap::new() }
    }

    /// Get a value by key
    pub fn get(&self, key: String) -> Option<String> {
        self.kv.get(&key).cloned()
    }

    /// Set a value by key
    pub fn set(&mut self, key: String, value: String) {
        self.kv.insert(key, value);
    }

    /// Remove a value by key
    pub fn remove(&mut self, key: String) {
        self.kv.remove(key.as_str());
    }
}
