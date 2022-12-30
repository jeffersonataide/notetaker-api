use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net;
use std::str::FromStr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "notetaker_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error while trying to connect to the database");
    tracing::debug!("Connected to the database: {:?}", pool);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = net::SocketAddr::from_str(&format!("0.0.0.0:{port}")).unwrap();
    tracing::debug!("Listening on: {addr}");

    axum::Server::bind(&addr)
        .serve(app(pool).into_make_service())
        .await
        .unwrap();
}

fn app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(handlers::hello_notetaker))
        .route("/hello_name", get(handlers::hello_name))
        .route(
            "/notes",
            get(handlers::list_notes).post(handlers::create_note),
        )
        .route("/notes/:note_id", get(handlers::get_note))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
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

        let app = app(pool);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, Notetaker!")
    }
}
