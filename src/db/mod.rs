pub mod boop;
pub mod entries;
pub mod guilds;
pub mod users;
use anyhow::Result;
use chrono::{prelude::*, Duration};
use poise::serenity_prelude::{self as serenity, Mentionable, UserId};
use std::sync::Arc;
use std::{thread, time};

use crate::{db, utils};
use sqlx::{Pool, Sqlite};
use std::env;

pub async fn new() -> Result<Pool<Sqlite>> {
    let db_url = env::var("DATABASE_URL").expect("No Database url found");

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            db_url
                .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                .create_if_missing(true),
        )
        .await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn poll(ctx: serenity::Context, db: Arc<Pool<Sqlite>>) -> Result<()> {
    let channel_id: serenity::ChannelId =
        utils::env_var("NOTIFICATION_CHANNEL").expect("No notification channel given");

    loop {
        let search = entries::fetch_entries(&db).await?;

        for entry in search {
            // Remind the user if the entries are due soon
            if entry.end_time - Utc::now() < Duration::minutes(30) && entry.remind {
                let user_id = db::users::get_user_from_db_id(&db, &entry.user_id)
                    .await?
                    .user_id;
                let user = UserId(user_id.try_into().unwrap())
                    .to_user(&ctx.http)
                    .await?;
                let response = format!("Do your entry: {}", entry.description);

                user.direct_message(&ctx.http, |m| {
                    m.embed(|e| e.title("Entry Reminder").description(response))
                })
                .await?;

                db::entries::complete_remind(&db, &entry.id).await?;
            }

            // Engage shaming
            if Utc::now() > entry.end_time && entry.active {
                let user_db_id = entry.user_id;
                let user_db_entry = db::users::get_user_from_db_id(&db, &user_db_id).await?;
                let user = serenity::UserId(
                    db::users::get_user_from_db_id(&db, &user_db_id)
                        .await?
                        .user_id
                        .try_into()
                        .unwrap(),
                );

                channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Expired Entry!").description(format!(
                                "
                                **User:** {} 
                                **Description:** {} 
                                **Total missed entries:** {} 

                                common {} L
                                ",
                                user.mention(),
                                entry.description,
                                user_db_entry.missed_entries.unwrap_or_else(|| 0),
                                user.mention()
                            ))
                        });
                        m
                    })
                    .await?;
                entries::complete_entry(&db, user).await?;
                users::increase_missed_entries(&db, &user).await?;
            }
        }
        thread::sleep(time::Duration::from_secs(30));
    }
}
