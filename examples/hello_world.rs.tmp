extern crate mita;
extern crate futures;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;

use mita::http::{HttpProto, HttpRequest, HttpResponse};
use futures::{future, Future, BoxFuture};
use tokio_service::Service;
use tokio_proto::TcpServer;

pub struct HelloWorld;

impl Service for HelloWorld {
    type Request = HttpRequest;
    type Response = HttpResponse;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        println!("Received: {},{},{}", req.verb, req.uri, req.version);

        // Create the response
        let resp = HttpResponse::new(200, "OK".into(), "HTTP/1.1".into());
        future::ok(resp).boxed()
    }
}

fn main() {
    let addr = "127.0.0.1:12345".parse().unwrap();
    let server = TcpServer::new(HttpProto, addr);

    server.serve(|| Ok(HelloWorld));
}
