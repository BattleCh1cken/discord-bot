use anyhow::Result;
use poise::serenity_prelude::{ChannelId, GuildId, RoleId};
use sqlx::{Pool, Sqlite};
#[derive(Debug)]
pub struct Guild {
    pub id: i32,
    pub guild_id: i64,
    pub reminder_master_role: Option<i64>,
    pub reminder_channel: Option<i64>,
}

impl Guild {
    pub fn merge(self, other: Guild) -> Self {
        Self {
            id: self.id,
            guild_id: self.guild_id,

            reminder_master_role: {
                if self.reminder_master_role == None {
                    other.reminder_master_role
                } else {
                    self.reminder_master_role
                }
            },
            reminder_channel: {
                if self.reminder_channel == None {
                    other.reminder_channel
                } else {
                    self.reminder_channel
                }
            },
        }
    }
}

pub async fn create_guild(db: &Pool<Sqlite>, guild: &GuildId) -> Result<()> {
    let mut conn = db.acquire().await?;
    let guild_id = *guild.as_u64() as i64;
    sqlx::query!(
        "
        insert into guilds (guild_id )
        select ?
        where not exists(select 1 from guilds where guild_id = ?);
                 ",
        guild_id,
        guild_id,
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn get_guild(db: &Pool<Sqlite>, guild: &GuildId) -> Result<Guild> {
    let mut conn = db.acquire().await?;
    let guild_id = *guild.as_u64() as i64;
    let guild = sqlx::query_as!(
        Guild,
        r#"
        select id as "id: i32", guild_id, reminder_master_role, reminder_channel from guilds
        where guild_id = ?
                               "#,
        guild_id
    )
    .fetch_one(&mut conn)
    .await?;

    Ok(guild)
}

pub async fn update_guilde_settings(db: &Pool<Sqlite>, guild: &Guild) -> Result<()> {
    let mut conn = db.acquire().await?;
    sqlx::query!("").execute(&mut conn).await?;

    Ok(())
}
