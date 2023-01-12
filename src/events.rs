use crate::{Data, Error};
use log::info;
use poise::serenity_prelude as serenity;
use poise::Event;
pub async fn event_listener(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            info!("{:#?} is coming online", data_about_bot.user.name);
        }
        _ => {
            info!("{}", event.name());
        }
    }
    Ok(())
}
