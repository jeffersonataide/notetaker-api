use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    id: String,
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNote {
    text: String,
}

pub async fn create_note(Json(payload): Json<CreateNote>) -> Json<Value> {
    let note = Note {
        id: "1".to_string(),
        text: payload.text,
    };

    Json(json!({
        "status": "created",
        "note": note
    }))
}

pub async fn list_notes() -> Json<Value> {
    let note = Note {
        id: "1".to_string(),
        text: "Test note".to_string(),
    };

    Json(json!({ "notes": vec![note] }))
}

pub async fn get_note(Path(note_id): Path<String>) -> Json<Value> {
    let note = Note {
        id: note_id,
        text: "sample note".to_string(),
    };

    Json(json!({ "note": note }))
}
