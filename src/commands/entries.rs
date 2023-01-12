use crate::{db, Context, Error};
use chrono::prelude::*;
use poise::serenity_prelude::{self as serenity, CacheHttp, Mentionable};

///Commands that handle notebook entries
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("create", "list", "complete")
)]
pub async fn entry(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

///create a new entry timer
#[poise::command(
    slash_command,
    prefix_command,
    //check = "crate::commands::check_if_is_notebooker"
)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "time you want the timer to run, in hours"] time: i64,
    #[description = "people you want to complete the entry"] user: serenity::User,
) -> Result<(), Error> {
    //We want to make sure that the use is supposed to be using this command
    if !ctx
        .author()
        .has_role(
            ctx.http(),
            *ctx.data().guild_id,
            *ctx.data().notebooker_role,
        )
        .await?
    {
        ctx.say("You aren't a notebooker").await?;
        return Ok(());
    }

    let current_time = Utc::now();
    //Change this to hours
    let duration = chrono::Duration::hours(time);
    let end_time = current_time + duration;

    db::entries::insert_entry(&ctx.data().database, &end_time, &user).await?;

    let response = format!(
        "Started an entry timer for {} lasting {} hours",
        user.mention(),
        time
    );
    ctx.say(response).await?;
    Ok(())
}

///Marks your entries as complete, absolves you of shame
#[poise::command(slash_command, prefix_command)]
pub async fn complete(ctx: Context<'_>) -> Result<(), Error> {
    let user: serenity::UserId = ctx.author().into();
    let user_id = *user.as_u64() as i64;
    db::entries::complete_entry(&ctx.data().database, user_id).await?;

    ctx.say("Entries marked as complete").await?;

    Ok(())
}

///This doesn't do anything yet
#[poise::command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("uuh I haven't implemented this yet").await?;
    Ok(())
}
