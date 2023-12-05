use crate::users::User;
use anyhow::Result;
use poise::serenity_prelude as serenity;
use sqlx::{Pool, Sqlite};

pub async fn search_for_score(db: &Pool<Sqlite>, user: serenity::UserId) -> Result<i64> {
    let user_id = *user.as_u64() as i64;
    let mut conn = db.acquire().await?;

    let query: i64 = sqlx::query_scalar!("select boop_score from users where user_id = ?", user_id)
        .fetch_one(&mut conn)
        .await?
        .unwrap_or(0);

    Ok(query)
}
pub async fn update_score(db: &Pool<Sqlite>, score: i64, user: serenity::UserId) -> Result<()> {
    let user_id = *user.as_u64() as i64;
    let mut conn = db.acquire().await?;
    sqlx::query!(
        "update users SET boop_score = ? WHERE user_id = ?",
        score,
        user_id
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

pub async fn get_top_scores(db: &Pool<Sqlite>) -> Result<Vec<User>> {
    let mut conn = db.acquire().await?;
    let scores  = sqlx::query_as!(
        User,
        r#"
        select
        id as "id!: i32", user_id as "user_id!: i64", boop_score as "boop_score? :i32", rps_wins as "rps_wins?: i32", missed_entries as "missed_entries?: i32"
        from users order by boop_score  desc limit 10
        "#,
    )
    .fetch_all(&mut conn)
    .await?;
    Ok(scores)
}
