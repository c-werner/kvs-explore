use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

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
    pub(crate) fn begin(&self) -> ReadyStore {
        new_unlocked(self)
    }
}

pub(crate) struct ReadyStore<'a> {
    store: &'a Store,
}

fn new_unlocked(s: &Store) -> ReadyStore {
    s.counter.fetch_add(1, Ordering::Relaxed);
    ReadyStore { store: s }
}

impl<'a> ReadyStore<'a> {
    pub(crate) fn count(&self) -> u64 {
        self.store.counter.load(Ordering::Relaxed)
    }

    pub(crate) fn keys(&self) -> Vec<String> {
        let map = self.store.map.read().unwrap();

        map.keys().cloned().collect()
    }

    fn get(&self, key: &str) -> Option<String> {
        let map = self.store.map.read().unwrap();

        match map.get(key) {
            Some(val) => Some(val.to_string()),
            None => None,
        }
    }

    fn set(&self, key: &str, value: &str) {
        let mut map = self.store.map.write().unwrap();

        map.insert(key.to_string(), value.to_string());
    }

    pub(crate) fn update(&self, key: &str, value: Option<String>) -> Option<String> {
        match value {
            Some(val) => {
                self.set(key, &val);
                Some(val)
            }
            None => self.get(key),
        }
    }

    pub(crate) fn has(&self, key: &str) -> bool {
        let map = self.store.map.read().unwrap();

        map.contains_key(key)
    }

    pub(crate) fn del(&self, key: &str) -> bool {
        let mut map = self.store.map.write().unwrap();

        match map.remove(key) {
            Some(_) => true,
            None => false,
        }
    }
}
