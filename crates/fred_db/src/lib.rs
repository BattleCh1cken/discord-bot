pub mod boop;
pub mod guilds;
pub mod reminders;
pub mod users;
use anyhow::Result;
use chrono::prelude::*;
use poise::serenity_prelude::{self as serenity, ChannelId, Mentionable, UserId};
use std::sync::Arc;
use std::{thread, time};

use sqlx::{Pool, Sqlite};
use std::env;

use self::guilds::get_guild_from_db_id;
use self::reminders::complete_reminder_remind;

pub async fn new() -> Result<Pool<Sqlite>> {
    let db_url = env::var("DATABASE_URL").expect("No Database url found");

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            db_url
                .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                .create_if_missing(true), // FIXME: The db isn't being created automatically
        )
        .await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn poll(ctx: serenity::Context, db: Arc<Pool<Sqlite>>) -> Result<()> {
    loop {
        let search = reminders::fetch_reminders(&db).await?;

        for reminder in search {
            // Remind the user that their time is nigh
            if let Some(reminder_time) = reminder.remind_time {
                if Utc::now() > reminder_time && reminder.active {
                    let user_id = users::get_user_from_db_id(&db, &reminder.user_id)
                        .await?
                        .user_id;
                    let user = UserId(user_id as u64).to_user(&ctx.http).await?;
                    let response = format!("Do your entry: {}", reminder.description);

                    user.direct_message(&ctx.http, |m| {
                        m.embed(|e| e.title("Reminder").description(response))
                    })
                    .await?;

                    complete_reminder_remind(&db, &user.into()).await?;
                }
            }

            // Engage shaming
            if Utc::now() > reminder.end_time && reminder.active {
                let user_db_id = reminder.user_id;
                let user_db_reminder = users::get_user_from_db_id(&db, &user_db_id).await?;
                let user = serenity::UserId(
                    users::get_user_from_db_id(&db, &user_db_id).await?.user_id as u64,
                );

                let reminder_channel = ChannelId(
                    get_guild_from_db_id(&db, &reminder.guild_id)
                        .await?
                        .reminder_channel
                        .unwrap() as u64,
                );

                reminder_channel
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Expired reminder!").description(format!(
                                "
                                **User:** {} 
                                **Description:** {} 
                                **Total missed reminders:** {} 
                                ",
                                user.mention(),
                                reminder.description,
                                user_db_reminder.missed_entries.unwrap_or(0),
                            ))
                        });
                        m
                    })
                    .await?;
                reminders::complete_reminder(&db, user).await?;
                users::increase_missed_entries(&db, &user).await?;
            }
        }
        thread::sleep(time::Duration::from_secs(30));
    }
}
