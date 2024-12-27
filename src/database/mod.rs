pub mod models;

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn get_pool() -> Result<SqlitePool, sqlx::Error> {
    // Create a connection pool

    // Get URL from environment variable
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
