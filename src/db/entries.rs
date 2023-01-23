use anyhow::{Context, Result};
use chrono::prelude::*;
use poise::serenity_prelude::UserId;
use sqlx::{FromRow, Pool, Sqlite};

use crate::db;

#[derive(Clone, Debug, FromRow)]
pub struct Entry {
    pub id: i32,
    pub end_time: DateTime<Utc>,
    pub user_id: i32,
    pub description: String,
    pub remind: bool,
    pub active: bool,
}

pub async fn insert_entry(
    db: &Pool<Sqlite>,
    end_time: &chrono::DateTime<Utc>,
    user_id: &i32,
    description: &String,
    remind: &bool,
) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query("INSERT INTO entries (end_time, user_id, description, remind, remind_time, active) VALUES(?, ?, ?, ?, ?, ?)")
        .bind(end_time)
        .bind(user_id)
        .bind(description)
        .bind(remind)
        .bind(0) //Placeholder value
        .bind(true)
        .execute(&mut conn)
        .await.context("Failed to insert new entry into database")?;

    Ok(())
}

pub async fn fetch_entries(db: &Pool<Sqlite>) -> Result<Vec<Entry>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as::<_, Entry>(
        "SELECT id, end_time, user_id, description, remind, active FROM entries;",
    )
    .fetch_all(&mut conn)
    .await
    .context("Failed to fetch all entries from database")?;
    Ok(search)
}
pub async fn fetch_active_entries(db: &Pool<Sqlite>) -> Result<Vec<Entry>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as::<_, Entry>(
        "SELECT id, end_time, user_id, description, remind, active FROM entries where active = true;",
    )
    .fetch_all(&mut conn)
    .await
    .context("Failed to fetch all entries from database")?;
    Ok(search)
}

pub async fn fetch_active_entries_for_user(db: &Pool<Sqlite>, db_id: &i32) -> Result<Vec<Entry>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as::<_, Entry>(
        "SELECT id, end_time, user_id, description, remind, active FROM entries WHERE user_id = ? and active = true;",
    )
    .bind(db_id)
    .fetch_all(&mut conn)
    .await
    .context("Failed to fetch active entries for user from database")?;

    Ok(search)
}
pub async fn fetch_entries_for_user(db: &Pool<Sqlite>, db_id: &i32) -> Result<Vec<Entry>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as::<_, Entry>(
        "SELECT id, end_time, user_id, description, remind, active FROM entries WHERE user_id = ?;",
    )
    .bind(db_id)
    .fetch_all(&mut conn)
    .await
    .context("Failed to fetch entries for user from database")?;

    Ok(search)
}

pub async fn complete_entry(db: &Pool<Sqlite>, user: UserId) -> Result<()> {
    let mut conn = db.acquire().await?;

    let user_db_id = db::users::get_user_from_id(&db, &user).await?.id;

    sqlx::query(
        "
        update entries
        set active = false
        where user_id = ?
        ",
    )
    .bind(user_db_id)
    .execute(&mut conn)
    .await
    .context("Failed to mark entry as complete in database")?;

    Ok(())
}

pub async fn complete_remind(db: &Pool<Sqlite>, entry_id: &i32) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query(
        "
        update entries
        set remind=false
        where id = ?
        ",
    )
    .bind(entry_id)
    .execute(&mut conn)
    .await
    .context("Failed to mark reminder as complete in database")?;

    Ok(())
}
