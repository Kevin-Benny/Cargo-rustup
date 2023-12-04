#![allow(unused)]

use axum::response::Html;
use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let var_name = Router::new();
    let routes_hello = var_name.route(
        "/hello",
        get(|| async{ Html("hello <strong> World!!!</strong>") }),
    );

    //region start server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    //end region start server
}
            