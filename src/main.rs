use axum::{routing::get, Router};
use std::net;
use std::str::FromStr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "notetaker_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = net::SocketAddr::from_str(&format!("0.0.0.0:{port}")).unwrap();
    tracing::debug!("Listening on: {addr}");

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(handlers::hello_notetaker))
        .route("/hello_name", get(handlers::hello_name))
        .route(
            "/notes",
            get(handlers::list_notes).post(handlers::create_note),
        )
        .route("/notes/:note_id", get(handlers::get_note))
        .layer(TraceLayer::new_for_http())
}
