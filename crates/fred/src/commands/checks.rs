use crate::{Context, Error};
use fred_db::guilds::get_guild;
use poise::serenity_prelude::{CacheHttp, RoleId};

pub async fn is_administrator(ctx: Context<'_>) -> Result<bool, Error> {
    let guild = ctx.guild().unwrap();
    let user = ctx.author();
    let result = guild
        .member_permissions(ctx, user.id)
        .await?
        .administrator();

    if !result {
        ctx.say("You aren't an administrator!").await?;
    }
    Ok(result)
}

pub async fn is_reminder_master(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = ctx.guild_id().unwrap();
    let reminder_master_role = match get_guild(&ctx.data().database, guild_id)
        .await?
        .reminder_master_role
    {
        Some(guild_id) => RoleId(guild_id as u64),
        None => {
            ctx.say("Your server does not have the reminder master role configured")
                .await?;
            return Ok(false);
        }
    };

    let result = ctx
        .author()
        .has_role(ctx.http(), guild_id, reminder_master_role)
        .await?;
    if !result {
        ctx.say("You need the reminder master role to be able to make reminders for other people")
            .await?;
    }
    Ok(result) // If result is false, check fails
}

pub async fn has_reminder_role_setting(ctx: Context<'_>) -> Result<bool, Error> {
    let guild = ctx.guild_id().unwrap();
    // TODO: These errors are essentially the same thing, combine them somehow
    // TODO: check if reminder_channel is valid (could have been deleted)
    match get_guild(&ctx.data().database, guild).await {
        Ok(settings) => {
            if settings.reminder_channel.is_some() {
                return Ok(true);
            }
            ctx.say("This server does not have the reminder channel configured.")
                .await?;
            return Ok(false);
        }
        Err(_) => {
            ctx.say("There are no settings for this server.").await?;
            return Ok(false);
        }
    };
}
