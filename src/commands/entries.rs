use crate::{db, Context, Error};
use chrono::prelude::*;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn create_entry(
    ctx: Context<'_>,
    #[description = "time you want the timer to run"] time: i64,
    #[description = "people you want to complete the entry"] user: serenity::User,
) -> Result<(), Error> {
    let current_time = Utc::now();
    let duration = chrono::Duration::seconds(time);
    let end_time = current_time + duration;
    let user_id = *user.id.as_u64() as i64;

    //make sure that this thread is the only one with access to the db
    let mut conn = ctx.data().database.acquire().await?;
    //TODO this should be a function
    //TODO figure out why id is always null
    sqlx::query!(
        "insert into entries (end_time, user_id, active) values(?, ?, ?)",
        end_time,
        user_id,
        true
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    let response = format!(
        "Started an entry timer for '{}' lasting {} seconds",
        user.name, time
    );
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn entry_complete(ctx: Context<'_>) -> Result<(), Error> {
    let mut conn = ctx.data().database.acquire().await?;
    let user: serenity::UserId = ctx.author().into();
    let user_id = *user.as_u64() as i64;

    sqlx::query!(
        "update entries set active=false
                        where entries.user_id = ?",
        user_id
    )
    .execute(&mut conn)
    .await?;
    ctx.say("Entries marked as complete").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn list_entries(ctx: Context<'_>) -> Result<(), Error> {
    let mut conn = ctx.data().database.acquire().await?;

    Ok(())
}
