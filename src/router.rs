use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use sqlx::{ Pool, Postgres};

use crate::handlers;

pub fn app(pool: Pool<Postgres>) -> Router {
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
