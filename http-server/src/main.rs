extern crate hyper;
extern crate futures;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use futures::future::FutureResult;
use hyper::Method::Post;
use hyper::{Body, Get, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};
use serde::de::IntoDeserializer;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{from_slice, json};
use std::convert::Infallible;
use crate::futures::Stream;
use crate::futures::Future;

#[derive(Clone, Copy)]
struct Echo;

#[derive(Deserialize, Debug)]
struct LuisPSarmientoRequest {
    token: String
}

//#[derive(Debug,Serialize, Deserialize)]
//struct Post {
//    title: String,
//    body: String,
//    userId: i32,
//}

impl Serialize for LuisPSarmientoRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("LuisPSarmientoRequest", 3)?;
        s.serialize_field("token", &self.token)?;
        /*s.serialize_field("age", &self.age)?;
        s.serialize_field("phones", &self.phones)?;*/
        s.end()
    }
}

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
            },
            (&Post, "/luispsarmiento/placeholder") => {
                let body = req.body().concat2().wait().unwrap_or_else(|_| hyper::Chunk::from(""));

                let parsed: LuisPSarmientoRequest = serde_json::from_slice(&body).unwrap();

                Response::new().with_header(ContentType::text()).with_status(StatusCode::Ok).with_body(parsed.token.into_bytes())
            },
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}

fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
