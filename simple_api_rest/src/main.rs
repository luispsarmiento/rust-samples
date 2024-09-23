use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_derive::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Bookmark {
    id: u32,
    url: String,
}

#[get("by-id/{id}")]
async fn bookmarks_by_id(path: web::Path<(u32,)>) -> impl Responder {
    let id = path.into_inner().0;

    let bookmark = Bookmark {
        id: id,
        url: "https://blog.x5ff.xyz".into(),
    };

    HttpResponse::Ok().json(bookmark)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    let helloWorld = std::env::var("HELLO_WORLD").expect("HELLO_WORLD must be set.");
    HttpResponse::Ok().body(helloWorld)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(bookmarks_by_id)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}