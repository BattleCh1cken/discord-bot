pub mod boop;
pub mod entries;
use anyhow::Result;
use chrono::prelude::*;
use poise::serenity_prelude::{self as serenity, Mentionable};
use std::sync::Arc;
use std::{thread, time};

use crate::utils;
use sqlx::{Pool, Sqlite};
use std::env;

pub async fn new() -> Result<Pool<Sqlite>> {
    let db_url = env::var("DATABASE_URL").expect("No Database url found");

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            db_url.parse::<sqlx::sqlite::SqliteConnectOptions>()?, //.create_if_missing(true),
        )
        .await?;
    //sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn poll(ctx: serenity::Context, db: Arc<Pool<Sqlite>>) -> Result<()> {
    let channel_id: serenity::ChannelId =
        utils::env_var("NOTIFICATION_CHANNEL").expect("No notif channel given");

    loop {
        let search = entries::fetch_entries(&db).await?;

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
                entries::complete_entry(&db, entry.user_id).await?;
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
