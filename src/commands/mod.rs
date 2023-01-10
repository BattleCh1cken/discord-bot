pub mod boop;
pub mod entries;

use crate::Context;
use crate::Error;

///show The Code
#[poise::command(slash_command)]
pub async fn repo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Take a look at my code: https://github.com/Area-53-Robotics/discord-bot")
        .await?;

    Ok(())
}
