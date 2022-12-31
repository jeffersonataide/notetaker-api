use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
}

pub async fn hello_notetaker() -> &'static str {
    "Hello, Notetaker!"
}

// Handle receiving query parameters
pub async fn hello_name(user: Query<User>) -> String {
    format!("Hello, {} \n", user.name)
}
