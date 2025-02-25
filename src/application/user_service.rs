// Entity และ business logic

use crate::domain::user::{User, UserInput};
use crate::infrastructure::db::DbPool;
use crate::infrastructure::jwt::{create_token, Claims};
use crate::domain::user::LoginResponse;


pub struct UserService;

impl UserService {
    pub async fn register(pool: &DbPool, input: UserInput) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *"
        )
        .bind(&input.username)
        .bind(&input.password) // ในโปรดักชันควร hash ก่อน
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn login(pool: &DbPool, input: UserInput) -> Result<LoginResponse, String> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(&input.username)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

        match user {
            Some(u) if u.password == input.password => {
                let claims = Claims { sub: u.username.clone(), exp: 10000000000 };
                let token = create_token(claims).map_err(|e| e.to_string())?;
                Ok(LoginResponse {
                    token,
                    username: u.username,
                    id: u.id,
                })
            }
            _ => Err("Invalid credentials".to_string()),
        }
    }
}