use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::net;
use std::str::FromStr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod handlers;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "notetaker_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let config = Config::new();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database.url)
        .await
        .expect("Error while trying to connect to the database");
    tracing::debug!("Connected to the database: {:?}", pool);

    let addr = net::SocketAddr::from_str(&format!("0.0.0.0:{}", config.server.port))
        .expect("Failed to create a socket address");
    tracing::debug!("Listening on: {addr}");

    axum::Server::bind(&addr)
        .serve(router::app(pool).into_make_service())
        .await
        .expect("Failed to start the server.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn hello_world() {
        let db_url = "postgres://postgres:postgres@localhost";

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .expect("Error while trying to connect to the database");

        let app = router::app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .expect("Failed to create a request"),
            )
            .await
            .expect("Failed to get a response");
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body())
            .await
            .expect("Failed to get response body");
        assert_eq!(&body[..], b"Hello, Notetaker!")
    }
}
