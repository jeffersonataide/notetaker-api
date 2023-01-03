use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct Note {
    id: i64,
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNote {
    content: String,
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateNote>,
) -> impl IntoResponse {
    //
    let result = sqlx::query_as!(
        Note,
        "INSERT INTO notes (content) VALUES ($1) RETURNING *;",
        payload.content
    )
    .fetch_one(&pool)
    .await
    .unwrap_or_else(|_| panic!("Could not get the list of notes"));
    tracing::debug!("Create note result: {:?}", result);

    (StatusCode::CREATED, Json(json!({ "note": result })))
}

pub async fn list(State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx::query_as!(Note, "SELECT * FROM notes;")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| panic!("Could not get the list of notes"));

    tracing::debug!("List notes result: {:?}", result);

    (StatusCode::OK, Json(json!({ "notes": result })))
}

pub async fn get(Path(note_id): Path<i64>, State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx::query_as!(Note, "SELECT * FROM notes WHERE id = $1;", note_id)
        .fetch_one(&pool)
        .await
        .unwrap_or_else(|_| panic!("Could not find a note where id={}", note_id));

    tracing::debug!("Get notes result: {:?}", result);

    (StatusCode::OK, Json(json!({ "note": result })))
}
