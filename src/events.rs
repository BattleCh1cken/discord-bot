use crate::{Data, Error};
use poise::serenity_prelude as serenity;
pub async fn event_listener(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        _ => {
            println!("{}", event.name());
        }
    }
    Ok(())
}
