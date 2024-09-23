extern crate hyper;
extern crate futures;
extern crate reqwest;
extern crate serde_json;
extern crate serde_derive;

use futures::future::FutureResult;
use hyper::{Get, StatusCode};
use hyper::header::{ContentType};
use hyper::server::{Http, Service, Request, Response};

#[derive(Clone, Copy)]
struct Echo;

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
