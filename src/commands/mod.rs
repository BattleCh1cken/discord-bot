pub mod fun;
pub mod misc;
pub mod owner;
pub mod reminder;
pub mod settings;

pub mod checks;

use crate::{Context, Error};

///show The Code
#[poise::command(slash_command)]
pub async fn repo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Take a look at my code: https://github.com/Area-53-Robotics/discord-bot")
        .await?;
    Ok(())
}
