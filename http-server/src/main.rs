extern crate hyper;
extern crate futures;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::{ thread, time };
use futures::future::FutureResult;
use hyper::Method::Post;
use hyper::{Get, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

fn heavy_work() -> String {
    let duration = time::Duration::from_millis(200);
    thread::sleep(duration);
    "done".to_string()
}

#[derive(Clone, Copy)]
struct Echo;

//#[derive(Debug,Serialize, Deserialize)]
//struct Post {
//    title: String,
//    body: String,
//    userId: i32,
//}

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/placeholder") => {
                let _body = reqwest::get("https://jsonplaceholder.typicode.com/todos").expect("Some wrong").text();
                Response::new()
                    .with_header(ContentType::json())
                    .with_body(_body.unwrap().to_string().into_bytes())
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}

fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
