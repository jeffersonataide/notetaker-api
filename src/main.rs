use axum::{routing::get, Router};
use std::net;

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handlers::hello_notetaker));

    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on: {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
