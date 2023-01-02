use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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

pub async fn create(Json(payload): Json<CreateNote>) -> Json<Value> {
    let note = Note {
        id: 1,
        content: payload.content,
    };

    Json(json!({
        "status": "created",
        "note": note
    }))
}

pub async fn list() -> Json<Value> {
    let note = Note {
        id: 1,
        content: "Test note".to_string(),
    };

    Json(json!({ "notes": vec![note] }))
}

pub async fn get(Path(note_id): Path<i64>, State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx::query_as!(Note, "SELECT * FROM notes WHERE id = $1;", note_id)
        .fetch_one(&pool)
        .await
        .unwrap_or_else(|_| panic!("Could not find a note where id={}", note_id));

    tracing::debug!("Result compiled: {:?}", result);

    (StatusCode::OK, Json(json!({ "note": result })))
}
