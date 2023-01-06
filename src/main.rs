mod commands;
mod events;
mod utils;
use commands::*;
use utils::env_var;
mod db;
use poise::serenity_prelude::{self as serenity, Activity};
use sqlx::{Pool, Sqlite};
use std::collections::HashSet;

// user data, which is stored and accessible in all command invocations
pub struct Data {
    pub database: Pool<Sqlite>,
    pub notebooker_role: serenity::RoleId,
}

//type aliases save us some typing
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let notebooker_role = env_var("NOTEBOOKER_ROLE");
    let guild_id = env_var("GUILD");
    let owner_id = env_var::<u64>("OWNER");
    let token = env_var::<String>("TOKEN");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), boop(), entries::entry()],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(events::event_listener(_ctx, event, _framework, _data))
            },
            owners: HashSet::from([serenity::UserId::from(owner_id.unwrap())]),
            ..Default::default()
        })
        .token(token.unwrap())
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_activity(Activity::watching("the watchmen")).await;
                //start the task to check if entries have expired
                let poll_ctx = ctx.clone();
                tokio::spawn(async move {
                    match db::poll(poll_ctx, db::new().await.unwrap()).await {
                        Ok(()) => {}
                        Err(error) => {
                            panic!("uh oh, we did an oopsy woopsy: {}", error)
                        }
                    };
                });

                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId(guild_id.unwrap()),
                )
                .await?;
                Ok(Data {
                    database: db::new().await?,
                    notebooker_role: notebooker_role.unwrap(),
                })
            })
        });

    framework.run().await.unwrap();
    Ok(())
}
