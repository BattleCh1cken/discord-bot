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
        Event::Message { new_message } => {
            let guild = fred_db::guilds::get_guild(
                &_user_data.database,
                new_message.guild(_ctx).unwrap().id,
            )
            .await?;

            let channel = new_message.channel(_ctx).await?.guild().unwrap();
            let messages = channel
                .messages(&_ctx, |retriever| retriever.before(new_message.id).limit(1))
                .await?;

            let last_message: i32 = messages[0].content.parse().unwrap();
            let desired_message = last_message + 1;

            if desired_message != new_message.content.parse::<i32>().unwrap() {
                new_message.delete(_ctx).await?;
            };
        }
        _ => {
            info!("{}", event.name());
        }
    }
    Ok(())
}
