mod domain;
mod application;
mod infrastructure;
mod delivery;

use crate::infrastructure::db::init_db_pool;
use crate::delivery::http::run_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db_pool().await;
    println!("Connected to database!");
    run_server(pool).await
}