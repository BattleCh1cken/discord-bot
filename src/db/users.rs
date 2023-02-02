use anyhow::Result;
use poise::serenity_prelude::UserId;
use sqlx::{FromRow, Pool, Sqlite};

use crate::db;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub user_id: i64,
    pub boop_score: i64,
    pub missed_entries: i32,
}

pub async fn create_user(db: &Pool<Sqlite>, user: &UserId) -> Result<()> {
    let mut conn = db.acquire().await?;
    let user_id = *user.as_u64() as i64;
    //Give the user a score of 0 if they don't have a score yet
    sqlx::query!(
        "
        insert into users (user_id, boop_score, missed_entries)
        select ?, ?, ?
        where not exists(select 1 from users where user_id = ?);
        ",
        user_id,
        0,
        0,
        user_id
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

pub async fn increase_missed_entries(db: &Pool<Sqlite>, user: &UserId) -> Result<()> {
    let mut conn = db.acquire().await?;

    let user_db_id = db::users::get_user_from_id(&db, &user).await?.id;
    sqlx::query(
        "update users set missed_entries = missed_entries + 1
                        where id = ?",
    )
    .bind(user_db_id)
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn get_user_from_id(db: &Pool<Sqlite>, user: &UserId) -> Result<User> {
    let mut conn = db.acquire().await?;
    let user_id = *user.as_u64() as i64;
    let user = sqlx::query_as::<_, User>(
        "select id, user_id, boop_score, missed_entries from users where user_id = ?",
    )
    .bind(user_id)
    .fetch_one(&mut conn)
    .await?;

    Ok(user)
}

pub async fn get_user_from_db_id(db: &Pool<Sqlite>, id: &i32) -> Result<User> {
    let mut conn = db.acquire().await?;
    let user = sqlx::query_as::<_, User>(
        "select id, user_id, boop_score, missed_entries from users where id = ?",
    )
    .bind(id)
    .fetch_one(&mut conn)
    .await?;

    Ok(user)
}
