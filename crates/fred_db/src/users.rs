use anyhow::Result;
use poise::serenity_prelude::UserId;
use sqlx::{Pool, Sqlite};

use crate::users;


#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub user_id: i64,
    pub boop_score: Option<i32>,
    pub rps_wins: Option<i32>,
    pub missed_entries: Option<i32>,
}

pub async fn create_user(db: &Pool<Sqlite>, user: &UserId) -> Result<()> {
    let mut conn = db.acquire().await?;
    let user_id = *user.as_u64() as i64;
    sqlx::query!(
        "
        insert into users (user_id )
        select ? 
        where not exists(select 1 from users where user_id = ?);
        ",
        user_id,
        user_id
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

pub async fn increase_missed_entries(db: &Pool<Sqlite>, user: &UserId) -> Result<()> {
    let mut conn = db.acquire().await?;

    let user_db_id = users::get_user_from_id(db, user).await?.id;
    sqlx::query!(
        "update users set missed_entries = missed_entries + 1
                        where id = ?",
        user_db_id
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn get_user_from_id(db: &Pool<Sqlite>, user: &UserId) -> Result<User> {
    let mut conn = db.acquire().await?;
    let user_id = *user.as_u64() as i64;
    let user = sqlx::query_as!(
        User,
        r#"select
        id as "id!: i32", user_id as "user_id: i64", boop_score as "boop_score? :i32", rps_wins as "rps_wins?: i32", missed_entries as "missed_entries?: i32"
        from users where user_id = ?"#,
        user_id
    )
    .fetch_one(&mut conn)
    .await?;

    Ok(user)
}

pub async fn get_user_from_db_id(db: &Pool<Sqlite>, id: &i64) -> Result<User> {
    let mut conn = db.acquire().await?;
    let user = sqlx::query_as!(
        User,
        r#"select
        id as "id!: i32", user_id as "user_id: i64", boop_score as "boop_score? :i32", rps_wins as "rps_wins?: i32", missed_entries as "missed_entries?: i32"
        from users where id = ?"#,
        id
    )
    .fetch_one(&mut conn)
    .await?;

    Ok(user)
}
