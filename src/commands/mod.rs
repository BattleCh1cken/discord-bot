pub mod boop;
pub mod entries;

use crate::{Context, Error};
use poise::serenity_prelude::CacheHttp;

///show The Code
#[poise::command(slash_command)]
pub async fn repo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Take a look at my code: https://github.com/Area-53-Robotics/discord-bot")
        .await?;
    Ok(())
}

//checks
async fn check_if_is_notebooker(ctx: Context<'_>) -> Result<bool, Error> {
    let not_notebooker = !ctx
        .author()
        .has_role(
            ctx.http(),
            *ctx.data().guild_id,
            *ctx.data().notebooker_role,
        )
        .await?;
    Ok(not_notebooker)
}
