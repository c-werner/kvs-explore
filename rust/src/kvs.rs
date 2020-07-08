use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) struct Store {
    pub(crate) counter: AtomicU64,
    pub(crate) map: RwLock<HashMap<String, String>>,
}

pub(crate) fn new() -> Store {
    Store {
        counter: AtomicU64::new(0),
        map: RwLock::new(HashMap::new()),
    }
}

impl Store {
    pub(crate) fn incr(&self) -> &Self {
        self.counter.fetch_add(1, Ordering::Relaxed);
        self
    }

    pub(crate) fn count(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }

    pub(crate) fn keys(&self) -> Vec<String> {
        let map = self.map.read().unwrap();

        map.keys().cloned().collect()
    }

    fn get(&self, key: &String) -> Option<String> {
        let map = self.map.read().unwrap();

        match map.get(key.as_str()) {
            Some(val) => Some(val.to_string()),
            None => None,
        }
    }

    fn set(&self, key: &String, value: &String) {
        let mut map = self.map.write().unwrap();

        map.insert(key.clone(), value.clone());
    }

    pub(crate) fn update(&self, key: &String, value: Option<String>) -> Option<String> {
        match value {
            Some(val) => {
                self.set(key, &val);
                Some(val)
            }
            None => self.get(key),
        }
    }

    pub(crate) fn has(&self, key: &String) -> bool {
        let map = self.map.read().unwrap();

        map.contains_key(key)
    }

    pub(crate) fn del(&self, key: &String) -> bool {
        let mut map = self.map.write().unwrap();

        match map.remove(key) {
            Some(_) => true,
            None => false
        }
    }
}