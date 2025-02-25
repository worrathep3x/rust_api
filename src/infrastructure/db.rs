// Postgres connection pool
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

pub type DbPool = PgPool;

pub async fn init_db_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}