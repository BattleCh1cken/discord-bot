use anyhow::Result;
use chrono::prelude::*;
use poise::serenity_prelude::{self as serenity, Mentionable};
use std::{thread, time};

use crate::utils;
use sqlx::{FromRow, Pool, Sqlite};
use std::env;

#[derive(Clone, Debug, FromRow)]
pub struct Entry {
    pub id: u32,
    pub end_time: DateTime<Utc>,
    pub user_id: i32,
    pub active: bool,
}

pub async fn new() -> Result<Pool<Sqlite>> {
    let db_url = env::var("DATABASE_URL").expect("No Database url found");

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            db_url
                .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                //.create_if_missing(true),
        )
        .await?;
    //sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn insert_entry(
    db: &Pool<Sqlite>,
    end_time: &chrono::DateTime<Utc>,
    user: &serenity::User,
) -> Result<()> {
    //make sure this thread is the only one with access to the db
    let mut conn = db.acquire().await?;
    let user_id = *user.id.as_u64() as i64; //sqlx doesn't like u64s
    sqlx::query("insert into entries (end_time, user_id, active) values(?, ?, ?)")
        .bind(end_time)
        .bind(user_id)
        .bind(true)
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn poll(ctx: serenity::Context, db: Pool<Sqlite>) -> Result<()> {
    let channel_id: serenity::ChannelId =
        utils::env_var("NOTIFICATION_CHANNEL").expect("No notif channel given");

    loop {
        let mut conn = db.acquire().await?;
        //TODO this should be a function
        let search =
            sqlx::query_as::<_, Entry>("SELECT id, end_time, user_id, active FROM entries;")
                .fetch_all(&mut conn)
                .await?;

        for entry in search {
            let current_time = Utc::now();
            if current_time > entry.end_time && entry.active {
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
                sqlx::query(
                    "update entries set active=false
                        where entries.id = ?",
                )
                .bind(entry.id)
                .execute(&mut conn)
                .await?;
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
