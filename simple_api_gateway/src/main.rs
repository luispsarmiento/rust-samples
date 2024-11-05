use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde_derive::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, RwLock};
use dotenv::dotenv;

async fn index(_req: HttpRequest) -> HttpResponse {
    let path = _req.path().to_string();

    HttpResponse::Ok().body(path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new().service(web::resource(r"/{service_uri:([a-zA-Z0-9._~:/?#@!$&'()*+,;=%-]*)?}").route(web::to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}