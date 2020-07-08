use std::collections::HashMap;
use std::sync::Mutex;

pub(crate) struct Store {
    pub(crate) counter: Mutex<i32>,
    pub(crate) map: Mutex<HashMap<String, String>>,
}

pub(crate) fn new() -> Store {
    Store {
        counter: Mutex::new(0),
        map: Mutex::new(HashMap::new()),
    }
}

impl Store {
    pub(crate) fn incr(&self) -> &Self {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        self
    }

    pub(crate) fn count(&self) -> i32 {
        *self.counter.lock().unwrap()
    }

    pub(crate) fn keys(&self) -> Vec<String> {
        let map = self.map.lock().unwrap();

        map.keys().map(|x| x.clone()).collect::<Vec<_>>()
    }

    fn get(&self, key: &String) -> Option<String> {
        let map = self.map.lock().unwrap();

        match map.get(key.as_str()) {
            Some(val) => Some(val.to_string()),
            None => None,
        }
    }

    fn set(&self, key: &String, value: &String) {
        let mut map = self.map.lock().unwrap();

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
        let map = self.map.lock().unwrap();

        map.contains_key(key)
    }

    pub(crate) fn del(&self, key: &String) -> bool {
        let mut map = self.map.lock().unwrap();

        match map.remove(key) {
            Some(_) => true,
            None => false
        }
    }
}