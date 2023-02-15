use anyhow::Result;
use chrono::prelude::*;
use poise::serenity_prelude::UserId;
use sqlx::{Pool, Sqlite};

use crate::db;

#[derive(Clone, Debug)]
pub struct Reminder {
    pub id: i64,
    pub end_time: DateTime<Utc>,
    pub user_id: i64,
    pub guild_id: i64,
    pub description: String,
    pub remind_time: Option<DateTime<Utc>>,
    pub active: bool,
}

pub async fn create_reminder(
    db: &Pool<Sqlite>,
    end_time: &DateTime<Utc>,
    user_id: &i32,
    guild_id: &i32,
    description: &String,
    remind_time: &Option<DateTime<Utc>>,
) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query!(
        "
        insert into reminders
        (end_time, user_id, guild_id, description,  remind_time, active)
        values(?, ?, ?, ?, ?, ?)
        ",
        end_time,
        user_id,
        guild_id,
        description,
        remind_time,
        true
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn fetch_reminders(db: &Pool<Sqlite>) -> Result<Vec<Reminder>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as!(
        Reminder,
        r#"select
        id, end_time as "end_time: DateTime<Utc>", user_id, guild_id, description, remind_time as "remind_time: DateTime<Utc>", active
        from reminders;"#,
    )
    .fetch_all(&mut conn)
    .await?;
    Ok(search)
}
pub async fn fetch_active_reminders(db: &Pool<Sqlite>) -> Result<Vec<Reminder>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as!(
        Reminder,
        r#"
        select
        id, end_time as "end_time: DateTime<Utc>", user_id, guild_id, description, remind_time as "remind_time: DateTime<Utc>", active
        from reminders where active = true;"#,
    )
    .fetch_all(&mut conn)
    .await?;
    Ok(search)
}

pub async fn fetch_active_reminders_for_user(
    db: &Pool<Sqlite>,
    db_id: &i32,
) -> Result<Vec<Reminder>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as!(Reminder,
        r#"
        select
        id, end_time as "end_time: DateTime<Utc>", user_id, guild_id, description, remind_time as "remind_time: DateTime<Utc>", active
        from reminders where user_id = ? and active = true;"#,
        db_id
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(search)
}
pub async fn fetch_reminders_for_user(db: &Pool<Sqlite>, db_id: &i32) -> Result<Vec<Reminder>> {
    let mut conn = db.acquire().await?;
    let search = sqlx::query_as!(Reminder,
        r#"
        select
        id, end_time as "end_time: DateTime<Utc>", user_id, guild_id, description, remind_time as "remind_time: DateTime<Utc>", active
        FROM reminders WHERE user_id = ?;"#,
        db_id
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(search)
}

pub async fn complete_reminder(db: &Pool<Sqlite>, user: UserId) -> Result<()> {
    let mut conn = db.acquire().await?;

    let user_db_id = db::users::get_user_from_id(db, &user).await?.id;

    sqlx::query!(
        "
        update reminders
        set active = false
        where user_id = ?
        ",
        user_db_id
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

// This is a good name
pub async fn complete_reminder_remind(db: &Pool<Sqlite>, user: &UserId) -> Result<()> {
    let mut conn = db.acquire().await?;

    let user_db_id = db::users::get_user_from_id(db, user).await?.id;

    sqlx::query!(
        "
        update reminders
        set remind_time = null
        where user_id = ?
        ",
        user_db_id
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}
