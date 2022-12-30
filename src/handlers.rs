use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn hello_notetaker() -> &'static str {
    "Hello, Notetaker!"
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
}

// Handle receiving query parameters
pub async fn hello_name(user: Query<User>) -> String {
    format!("Hello, {} \n", user.name)
}

#[derive(Debug, Serialize)]
pub struct Note {
    id: i32,
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNote {
    text: String,
}

pub async fn create_note(Json(payload): Json<CreateNote>) -> Json<Value> {
    let note = Note {
        id: 1,
        text: payload.text,
    };

    Json(json!({
        "status": "created",
        "note": note
    }))
}

pub async fn list_notes() -> Json<Value> {
    let note = Note {
        id: 1,
        text: "Test note".to_string(),
    };

    Json(json!({ "notes": vec![note] }))
}

pub async fn get_note(Path(note_id): Path<i32>, State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx::query_as!(Note, "SELECT * FROM notes WHERE id = $1;", note_id)
        .fetch_one(&pool)
        .await
        .unwrap_or_else(|_| panic!("Could not find a note where id={}", note_id));

    tracing::debug!("Result compiled: {:?}", result);

    (StatusCode::OK, Json(json!({ "note": result })))
}
