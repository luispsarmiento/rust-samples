extern crate hyper;
extern crate futures;
extern crate tokio;
extern crate hyper_util;

use std::{ thread, time };
use hyper::body::{Body, Incoming};
use hyper::{Error, Method, Request, Response, StatusCode};
use hyper::service::{service_fn, Service};
use futures::{future};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use tokio::{net::TcpListener, task::JoinSet};

fn heavy_work() -> String {
    let duration = time::Duration::from_millis(200);
    thread::sleep(duration);
    "done".to_string()
}

#[derive(Clone, Copy)]
struct Echo;

impl Service<Request<Incoming>> for Echo {
    type Response = Response<T>;
    type Error = Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        futures::future::ok(match (req.method(), req.uri().path()) {
            (&Method::GET, "/data") => {
                let b = heavy_work().into_bytes();
                Response::new(b)
            }
            _ => {
                let mut res = Response::default();
                *res.status_mut() = StatusCode::NOT_FOUND;
                res
            },
        })
    }
}

#[tokio::main]
async fn main() {
    //let addr = "0.0.0.0:3000".parse().unwrap();
    //let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    //server.run().unwrap();
    let listen_addr = "127.0.0.1:8000";
    let tcp_listener = TcpListener::bind(listen_addr).await.unwrap();
    println!("listening on http://{listen_addr}");

    let mut join_set = JoinSet::new();
    loop {
        let (stream, addr) = match tcp_listener.accept().await {
            Ok(x) => x,
            Err(e) => {
                eprintln!("failed to accept connection: {e}");
                continue;
            }
        };

        let serve_connection = async move {
            println!("handling a request from {addr}");

            let result = Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(stream), Echo)
                .await;

            if let Err(e) = result {
                eprintln!("error serving {addr}: {e}");
            }

            println!("handled a request from {addr}");
        };

        join_set.spawn(serve_connection);
    }
}
