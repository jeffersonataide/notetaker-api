use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;

mod hello;
mod notes;

pub fn app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(hello::hello_notetaker))
        .route("/hello_name", get(hello::hello_name))
        .route("/notes", get(notes::list).post(notes::create))
        .route("/notes/:note_id", get(notes::get))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}
