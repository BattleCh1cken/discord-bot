mod commands;
mod events;
mod utils;
use commands::*;
use poise::serenity_prelude::{self as serenity, Activity};
use sqlx::{Pool, Sqlite};
use std::collections::HashSet;
use std::sync::Arc;
use utils::env_var;
mod db;
use log::error;

// user data, which is stored and accessible in all command invocations
#[derive(Debug)]
pub struct Data {
    pub database: Arc<Pool<Sqlite>>,
    pub notebooker_role: Arc<serenity::RoleId>,
    pub guild_id: Arc<serenity::GuildId>,
}

//type aliases save us some typing
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let notebooker_role = Arc::new(serenity::RoleId(env_var("NOTEBOOKER_ROLE").unwrap()));
    let guild_id = Arc::new(serenity::GuildId(env_var("GUILD").unwrap()));
    let owner_id = env_var::<u64>("OWNER");
    let token = env_var::<String>("TOKEN");
    let database = Arc::new(db::new().await?);

    env_logger::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                repo(),
                boop::boop(),
                boop::leaderboard(),
                entries::entry(),
                owner::register(),
                owner::motivate(),
                misc::help(),
            ],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(events::event_listener(_ctx, event, _framework, _data))
            },
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = poise::builtins::on_error(error).await {
                        error!("Error while handling error: {}", e);
                    }
                })
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
                let polling_ctx = ctx.clone();
                let polling_database = Arc::clone(&database);
                tokio::spawn(async move {
                    match db::poll(polling_ctx, polling_database).await {
                        Ok(()) => {}
                        Err(error) => {
                            error!("Error while polling: {}", error);
                            panic!("Error while Polling: {}", error)
                        }
                    };
                });

                poise::builtins::register_in_guild(ctx, &framework.options().commands, *guild_id)
                    .await?;
                Ok(Data {
                    database,
                    notebooker_role,
                    guild_id,
                })
            })
        });

    framework.run().await.unwrap();
    Ok(())
}
