use axum::{routing::get, Router};

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handlers::hello_notetaker));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
