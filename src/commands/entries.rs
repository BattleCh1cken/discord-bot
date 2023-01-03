use crate::{db, Context, Error};
use chrono::prelude::*;
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("create", "list", "complete")
)]

///Commands that handle notebook entries
pub async fn entry(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Subcommands include: create, complete, list")
        .await?;
    Ok(())
}

///create a new entry timer
#[poise::command(slash_command, prefix_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "time you want the timer to run"] time: i64,
    #[description = "people you want to complete the entry"] user: serenity::User,
) -> Result<(), Error> {
    let current_time = Utc::now();
    let duration = chrono::Duration::seconds(time);
    let end_time = current_time + duration;

    db::insert_entry(&ctx.data().database, &end_time, &user).await?;

    let response = format!(
        "Started an entry timer for '{}' lasting {} seconds",
        user.name, time
    );
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn complete(ctx: Context<'_>) -> Result<(), Error> {
    let mut conn = ctx.data().database.acquire().await?;
    let user: serenity::UserId = ctx.author().into();
    let user_id = *user.as_u64() as i64;
    //TODO make this a function
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
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("uuh I haven't implemented this yet").await?;
    Ok(())
}
