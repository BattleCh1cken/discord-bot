use crate::db::users::User;
use anyhow::Result;
use poise::serenity_prelude as serenity;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Debug, FromRow)]
pub struct BoopScore {
    pub id: i32,
    pub score: i64,
    pub user_id: i64,
}
pub async fn search_for_score(db: &Pool<Sqlite>, user: serenity::UserId) -> Result<i64> {
    let user_id = *user.as_u64() as i64;
    let mut conn = db.acquire().await?;

    let query: i64 = sqlx::query_scalar("SELECT boop_score FROM users WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&mut conn)
        .await?;

    Ok(query)
}
pub async fn update_score(db: &Pool<Sqlite>, score: i64, user: serenity::UserId) -> Result<()> {
    let user_id = *user.as_u64() as i64;
    let mut conn = db.acquire().await?;
    sqlx::query("UPDATE users SET boop_score = ? WHERE user_id = ?")
        .bind(score)
        .bind(user_id)
        .execute(&mut conn)
        .await?;
    Ok(())
}

pub async fn get_top_scores(db: &Pool<Sqlite>) -> Result<Vec<User>> {
    let mut conn = db.acquire().await?;
    let scores = sqlx::query_as::<_, User>(
        "
        SELECT id, user_id, boop_score, missed_entries FROM users ORDER BY boop_score DESC limit 10
        ",
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(scores)
}
