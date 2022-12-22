mod commands;
use commands::*;
mod db;
use dotenvy;
use poise::serenity_prelude::{self as serenity, Activity};
use sqlx::{Pool, Sqlite};
use std::collections::HashSet;

pub struct Data {
    pub database: Pool<Sqlite>,
    pub notebooker_role: serenity::RoleId,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

fn env_var<T: std::str::FromStr>(name: &str) -> Result<T, Error>
where
    T::Err: std::fmt::Display,
{
    Ok(std::env::var(name)
        .map_err(|_| format!("Missing {}", name))?
        .parse()
        .map_err(|e| format!("Invalid {}: {}", name, e))?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //load ev vars
    dotenvy::dotenv()?;
    let notebooker_role = env_var("NOTEBOOKER_ROLE");
    let guild_id = env_var("GUILD");
    let owner_id = env_var::<u64>("OWNER");
    let token = env_var::<String>("TOKEN");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                age(),
                boop(),
                entries::create_entry(),
                entries::list_entries(),
            ],
            owners: HashSet::from([serenity::UserId::from(owner_id.unwrap())]),
            ..Default::default()
        })
        .token(token.unwrap())
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_activity(Activity::watching("the watchmen")).await;
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
