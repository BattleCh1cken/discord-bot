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

    //Give the user a score of 0 if they don't have a score yet
    sqlx::query(
        "
        INSERT INTO boop_score (score,user_id)
        SELECT ?, ?
        WHERE NOT EXISTS(SELECT 1 FROM boop_score WHERE user_id = ?);
        ",
    )
    .bind(0)
    .bind(user_id)
    .bind(user_id)
    .execute(&mut conn)
    .await?;

    let query: i64 = sqlx::query_scalar("SELECT score FROM boop_score WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&mut conn)
        .await?;

    Ok(query)
}
pub async fn update_score(db: &Pool<Sqlite>, score: i64, user: serenity::UserId) -> Result<()> {
    let user_id = *user.as_u64() as i64;
    let mut conn = db.acquire().await?;
    sqlx::query("UPDATE boop_score SET score = ? WHERE user_id = ?")
        .bind(score)
        .bind(user_id)
        .execute(&mut conn)
        .await?;
    Ok(())
}

pub async fn get_top_scores(db: &Pool<Sqlite>) -> Result<Vec<BoopScore>> {
    let mut conn = db.acquire().await?;
    let scores = sqlx::query_as::<_, BoopScore>(
        "
SELECT id, score, user_id FROM boop_score ORDER BY score DESC limit 10
        ",
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(scores)
}
