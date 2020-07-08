use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;

struct KVS {
    counter: Mutex<i32>,
    map: Mutex<HashMap<String, String>>,
}

impl KVS {
    fn incr(&self) -> &Self {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        self
    }

    fn count(&self) -> i32 {
        *self.counter.lock().unwrap()
    }

    fn keys(&self) -> Vec<String> {
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

    fn update(&self, key: &String, value: Option<String>) -> Option<String> {
        match value {
            Some(val) => {
                self.set(key, &val);
                Some(val)
            }
            None => self.get(key),
        }
    }

    fn has(&self, key: &String) -> bool {
        let map = self.map.lock().unwrap();

        map.contains_key(key)
    }

    fn del(&self, key: &String) -> bool {
        let mut map = self.map.lock().unwrap();

        match map.remove(key) {
            Some(_) => true,
            None => false
        }
    }
}

#[get("/")]
async fn index(data: web::Data<KVS>) -> String {
    format!("Request number: {}", data.incr().count())
}

async fn list_keys(data: web::Data<KVS>) -> String {
    data.keys().join("\n")
}

#[derive(Deserialize)]
struct KeyInfo {
    key: String,
}

#[derive(Deserialize)]
struct ValueInfo {
    v: Option<String>,
}

async fn get_or_update_key(
    data: web::Data<KVS>,
    key_info: web::Path<KeyInfo>,
    query_info: web::Query<ValueInfo>,
) -> String {

    /* The query_info.v.clone() here feels wrong but I'm not sure
        what the right answer is for this.
        Honestly, I'm confused why I can't send query_info.v
        *without* cloning it
     */
    let result = data
        .incr()
        .update(&key_info.key, query_info.v.clone());

    match result {
        Some(val) => val,
        None => format!("'{}' not found", &key_info.key),
    }
}

async fn del_key(data: web::Data<KVS>, key_info: web::Path<KeyInfo>) -> String {
    data.incr().del(&key_info.key).to_string()
}

async fn has_key(data: web::Data<KVS>, key_info: web::Path<KeyInfo>) -> String {
    data.incr().has(&key_info.key).to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(KVS {
        counter: Mutex::new(0),
        map: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(index)
            .route("/list", web::get().to(list_keys))
            .route("/k/{key}", web::get().to(get_or_update_key))
            .route("/h/{key}", web::get().to(has_key))
            .route("/d/{key}", web::get().to(del_key))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
