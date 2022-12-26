use chrono::prelude::*;
use poise::serenity_prelude::{self as serenity, Mentionable};
use std::{thread, time};

use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

#[derive(Clone, Debug)]
pub struct Entry {
    pub id: u32,
    pub end_time: DateTime<Utc>,
    pub member_id: u32,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub user_id: i64,
}

pub async fn new() -> anyhow::Result<Pool<Sqlite>> {
    let db = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn poll(ctx: serenity::Context, db: Pool<Sqlite>) -> anyhow::Result<()> {
    //TODO don't hardcode this
    let channel_id = serenity::model::id::ChannelId(823988576709640264);

    loop {
        let mut conn = db.acquire().await?;
        //TODO this should be a function
        let search = sqlx::query!("SELECT id, end_time, user_id, active FROM entries;")
            .fetch_all(&mut conn)
            .await?;

        for entry in search {
            let current_time = Utc::now();
            if current_time.naive_utc() > entry.end_time && entry.active == true {
                let user = serenity::UserId(entry.user_id.try_into().unwrap());

                channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Expired Entry!").description(format_args!(
                                "Oops! Looks like {} forgot to complete their entry in time!",
                                user.mention()
                            ))
                        });
                        m
                    })
                    .await?;
                //TODO this should be a function
                sqlx::query!(
                    "update entries set active=false
                        where entries.id = ?",
                    entry.id
                )
                .execute(&mut conn)
                .await?;
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
