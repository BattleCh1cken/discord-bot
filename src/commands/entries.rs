use crate::{
    db::{self, entries::*},
    Context, Error,
};
use chrono::prelude::*;
use poise::serenity_prelude::{self as serenity, CacheHttp, Mentionable, UserId};

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
    #[description = "person you want to complete the entry"] user: serenity::User,
    #[description = "what you want them to write in that entry"] description: String,
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

    db::entries::insert_entry(&ctx.data().database, &end_time, &user, &description).await?;

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Entry timer created").description(format!(
                "
                User: {}
                Time to complete: {} hours
                Description: {}
                ",
                user.mention(),
                time,
                description
            ))
        })
    })
    .await?;
    Ok(())
}

///Marks your entries as complete, absolves you of shame
#[poise::command(slash_command, prefix_command)]
pub async fn complete(ctx: Context<'_>) -> Result<(), Error> {
    let user: UserId = ctx.author().into();
    let user_id = *user.as_u64() as i64;
    db::entries::complete_entry(&ctx.data().database, user_id).await?;

    ctx.say("Entries marked as complete").await?;

    Ok(())
}

///Displays the current entries you need to do
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author().id;
    let entries = fetch_entries_for_user(&ctx.data().database, &user).await?;
    let mut response = String::new();
    let mut index = 0;
    for entry in entries {
        if entry.active {
            index += 1;
            let time_left = entry.end_time - Utc::now();
            response += &format!(
                "{index}. {} -- time left - {}:{}\n",
                entry.description,
                time_left.num_hours(),
                time_left.num_minutes()
            )
        }
    }
    ctx.send(|m| m.embed(|e| e.title("Entries due").description(response)))
        .await?;
    Ok(())
}
