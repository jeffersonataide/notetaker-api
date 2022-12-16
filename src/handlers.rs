use axum::extract::Query;
use serde::Deserialize;

pub async fn hello_notetaker() -> &'static str {
    return "Hello, Notetaker!";
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
}

// Handle receiving query parameters
pub async fn hello_name(user: Query<User>) -> String {
    return format!("Hello, {} \n", user.name);
}
