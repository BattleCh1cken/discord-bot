use chrono::Utc;
use humantime::format_duration;

use crate::{Context, Error};
/// Shows help menu
#[poise::command(prefix_command, track_edits, slash_command, category = "Miscellaneous")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ephemeral: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
/// Returns the time the bot has been online
#[poise::command(prefix_command, track_edits, slash_command, category = "Miscellaneous")]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = format_duration((Utc::now() - ctx.data().start_time).to_std().unwrap());
    ctx.say(format!("{} seconds", uptime)).await?;

    Ok(())
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum Resources {
    Programming,
    Building,
    Notebooking,
    General,
}
/// Returns the time the bot has been online
#[poise::command(prefix_command, track_edits, slash_command, category = "Miscellaneous")]
pub async fn resources(
    ctx: Context<'_>,
    #[description = "bob"] resource: Option<Resources>,
) -> Result<(), Error> {
    let resource = resource.unwrap_or_else(|| Resources::General);

    let response = match resource {
        Resources::General => String::from("
                                           [link](https://docs.google.com/document/d/1j0PSzMkfBmEd7lpYY5uJBVKYtn6knZgzpn78Stlz3rk/edit?usp=sharing)
                                           "),
        Resources::Building => String::from("e"),
        Resources::Programming => String::from("e"),
        Resources::Notebooking => String::from("e"),
    };
    ctx.say(response).await?;
    Ok(())
}
