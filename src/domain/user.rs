use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String, // ในโปรดักชันควร hash password
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub id: i32,
}