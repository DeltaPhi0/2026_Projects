use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub async fn init_db(db_path: &str) -> SqlitePool {
    let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))
        .expect("Invalid DB connection string")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options)
        .await
        .expect("DB connection failed");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                message TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
    )
    .execute(&pool)
    .await
    .expect("DB init failed");

    pool
}
