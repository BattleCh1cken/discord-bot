use crate::{Context, Error};
use poise::serenity_prelude::{CacheHttp, User};

///Makes slash commands available
#[poise::command(prefix_command, slash_command, owners_only, ephemeral, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Motivation moment
#[poise::command(slash_command, owners_only, ephemeral, hide_in_help)]
pub async fn motivate(
    ctx: Context<'_>,
    #[description = "who you want to motivate"] user: Option<User>,
    #[description = "message you want to send"] message: String,
) -> Result<(), Error> {
    match user {
        Some(user) => {
            user.direct_message(&ctx.http(), |m| m.content(message))
                .await?;
            ctx.say("message sent").await?;
        }
        None => {
            let channel = ctx.channel_id();
            channel.send_message(ctx, |m| m.content(message)).await?;
            ctx.say("message sent").await?;
        }
    }

    Ok(())
}
