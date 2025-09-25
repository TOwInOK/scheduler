use std::str::FromStr;

use sqlx::{Pool, Sqlite, SqlitePool, query, sqlite::SqliteConnectOptions};

use crate::error::Result;

pub async fn execute_pool() -> Result<Pool<Sqlite>> {
    let opt =
        SqliteConnectOptions::from_str("sqlite://scheduler_store.db")?.create_if_missing(true);
    Ok(SqlitePool::connect_with(opt).await?)
}

pub async fn create_default_if_not_exists(pool: Pool<Sqlite>) -> Result<()> {
    query!(
        "CREATE TABLE IF NOT EXISTS users (
                id   INTEGER PRIMARY KEY,
                selected_group INTEGER NOT NULL
            )",
    )
    .execute(&pool)
    .await?;
    Ok(())
}
