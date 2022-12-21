mod commands;
mod db;
use dotenvy;
use poise::serenity_prelude as serenity;
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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::age(),
                commands::boop(),
                commands::entries::create_entry(),
            ],
            owners: HashSet::from([serenity::UserId::from(owner_id.unwrap())]),
            ..Default::default()
        })
        .token(std::env::var("TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
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