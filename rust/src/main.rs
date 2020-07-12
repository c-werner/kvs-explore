mod kvs;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CountResp {
    request_number: u64,
}

#[derive(Serialize, Deserialize)]
struct ListResp {
    keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GetOrUpdateResp {
    key: String,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct BoolResp {
    key: String,
    ok: bool,
}

#[get("/")]
async fn index(data: web::Data<kvs::Store>) -> HttpResponse {
    HttpResponse::Ok().json(CountResp {
        request_number: data.begin().count(),
    })
}

async fn list_keys(data: web::Data<kvs::Store>) -> HttpResponse {
    HttpResponse::Ok().json(ListResp {
        keys: data.begin().keys(),
    })
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
) -> Result<HttpResponse> {
    let result = data
        .begin()
        .update(&key_info.key, query_info.into_inner().v);

    match result {
        Some(val) => Ok(HttpResponse::Ok().json(GetOrUpdateResp {
            key: key_info.into_inner().key,
            value: Some(val),
        })),
        None => Ok(HttpResponse::NotFound().json(GetOrUpdateResp {
            key: key_info.into_inner().key,
            value: None,
        })),
    }
}

async fn del_key(data: web::Data<kvs::Store>, key_info: web::Path<KeyInfo>) -> HttpResponse {
    HttpResponse::Ok().json(BoolResp {
        ok: data.begin().del(&key_info.key),
        key: key_info.into_inner().key,
    })
}

async fn has_key(data: web::Data<kvs::Store>, key_info: web::Path<KeyInfo>) -> HttpResponse {
    HttpResponse::Ok().json(BoolResp {
        ok: data.begin().has(&key_info.key),
        key: key_info.into_inner().key,
    })
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
