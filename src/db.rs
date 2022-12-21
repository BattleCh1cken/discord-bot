use chrono::prelude::*;
use sqlx::{FromRow, Pool, Sqlite, SqlitePool};
use std::env;

#[derive(Clone, FromRow, Debug)]
pub struct Entry {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Clone, FromRow, Debug)]
pub struct Member {
    pub user_id: i64,
    pub name: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct MemberEntries {
    pub id: i32,
    pub member_id: i32,
    pub entery_id: i32,
}

pub async fn new() -> anyhow::Result<Pool<Sqlite>> {
    let db = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn poll(db: &Pool<Sqlite>) -> anyhow::Result<()> {
    Ok(())
}
