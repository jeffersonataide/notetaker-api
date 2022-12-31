use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;

use crate::handlers;

mod hello;

pub fn app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(hello::hello_notetaker))
        .route("/hello_name", get(hello::hello_name))
        .route(
            "/notes",
            get(handlers::list_notes).post(handlers::create_note),
        )
        .route("/notes/:note_id", get(handlers::get_note))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}
