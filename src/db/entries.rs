use anyhow::Result;
use chrono::prelude::*;
use poise::serenity_prelude as serenity;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Clone, Debug, FromRow)]
pub struct Entry {
    pub id: i32,
    pub end_time: DateTime<Utc>,
    pub user_id: i64,
    pub active: bool,
}

pub async fn insert_entry(
    db: &Pool<Sqlite>,
    end_time: &chrono::DateTime<Utc>,
    user: &serenity::User,
) -> Result<()> {
    let mut conn = db.acquire().await?;
    let user_id = *user.id.as_u64() as i64; //sqlx doesn't like u64s
    sqlx::query("INSERT INTO entries (end_time, user_id, active) VALUES(?, ?, ?)")
        .bind(end_time)
        .bind(user_id)
        .bind(true)
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn fetch_entries(db: &Pool<Sqlite>) -> Result<Vec<Entry>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as::<_, Entry>("SELECT id, end_time, user_id, active FROM entries;")
        .fetch_all(&mut conn)
        .await?;
    Ok(search)
}

pub async fn complete_entry(db: &Pool<Sqlite>, user: i64) -> Result<()> {
    let mut conn = db.acquire().await?;

    sqlx::query(
        "update entries set active=false
                        where entries.user_id = ?",
    )
    .bind(user)
    .execute(&mut conn)
    .await?;

    Ok(())
}
