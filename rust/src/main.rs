mod kvs;

use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;

#[get("/")]
async fn index(data: web::Data<kvs::Store>) -> String {
    format!("Request number: {}", data.incr().count())
}

async fn list_keys(data: web::Data<kvs::Store>) -> String {
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
    data: web::Data<kvs::Store>,
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

async fn del_key(data: web::Data<kvs::Store>, key_info: web::Path<KeyInfo>) -> String {
    data.incr().del(&key_info.key).to_string()
}

async fn has_key(data: web::Data<kvs::Store>, key_info: web::Path<KeyInfo>) -> String {
    data.incr().has(&key_info.key).to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(kvs::new());

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
