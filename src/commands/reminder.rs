use crate::{
    commands::checks::{has_reminder_role_setting, is_reminder_master},
    db::{self, guilds::get_guild, reminders::*, users},
    Context, Error,
};
use chrono::prelude::*;
use humantime::{self, format_duration};
use poise::serenity_prelude::{self as serenity, CacheHttp, Mentionable, UserId};

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("create", "list", "complete")
)]
pub async fn reminder(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

///create a new reminder
#[poise::command(
    slash_command,
    prefix_command,
    check = "has_reminder_role_setting",
    guild_only
)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "time you want the timer to run, in hours"] time: i64,
    #[description = "person you want to complete the reminder"] user: Option<serenity::User>,
    #[description = "what the reminder is for"] description: String,
    #[description = "The time before the exiration time the reminder should be triggered."]
    remind_time: Option<i32>,
) -> Result<(), Error> {
    let current_time = Utc::now();
    let duration = chrono::Duration::hours(time);
    let end_time = current_time + duration;

    let remind_time = remind_time.map(|time| end_time - chrono::Duration::hours(time as i64));

    let user = match user.as_ref() {
        None => ctx.author(),
        Some(target_user) => {
            if !is_reminder_master(ctx).await? {
                return Ok(());
            }
            target_user
        }
    };

    // Clone is needed because user is cast to i64, which requires a dereference
    db::users::create_user(&ctx.data().database, &user.clone().into()).await?;

    let user_id = db::users::get_user_from_id(&ctx.data().database, &user.clone().into()).await?;
    let guild_db_id = get_guild(&ctx.data().database, &ctx.guild_id().unwrap())
        .await?
        .id;

    db::reminders::create_reminder(
        &ctx.data().database,
        &end_time,
        &user_id.id,
        &guild_db_id,
        &description,
        &remind_time,
    )
    .await?;

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Reminder Created").description(format!(
                "
                **User:** {}
                **Time to complete:** {}
                **Description:** {}
                **Remind:** {}
                ",
                user.mention(),
                format_duration(duration.to_std().unwrap()),
                description,
                match remind_time {
                    None => "false".to_string(),
                    Some(remind_time) => {
                        format_duration((end_time - remind_time).to_std().unwrap()).to_string()
                    }
                },
            ))
        })
    })
    .await?;
    Ok(())
}

///Marks your reminders as complete, absolves you of shame
#[poise::command(slash_command, prefix_command)]
pub async fn complete(ctx: Context<'_>) -> Result<(), Error> {
    let user: UserId = ctx.author().into();
    db::reminders::complete_reminder(&ctx.data().database, user).await?;

    ctx.say("Reminders marked as complete").await?;

    Ok(())
}

#[derive(poise::ChoiceParameter, Debug)]
pub enum ViewOptions {
    #[name = "all"]
    All,
    #[name = "self: active and expired"]
    AuthorExpired,
    #[name = "all: active and expired"]
    AllExpired,
    #[name = "self"]
    Author,
}
//Displays reminders
#[poise::command(slash_command, prefix_command)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "whether to display reminders that are expired"] view: Option<ViewOptions>,
) -> Result<(), Error> {
    let user = ctx.author().id;

    let reminders = match view.unwrap_or(ViewOptions::Author) {
        ViewOptions::Author => {
            let db_user_id = db::users::get_user_from_id(&ctx.data().database, &user)
                .await?
                .id;
            fetch_active_reminders_for_user(&ctx.data().database, &db_user_id).await?
        }
        ViewOptions::AuthorExpired => {
            let db_user_id = db::users::get_user_from_id(&ctx.data().database, &user)
                .await?
                .id;
            fetch_reminders_for_user(&ctx.data().database, &db_user_id).await?
        }
        ViewOptions::All => fetch_active_reminders(&ctx.data().database).await?,

        ViewOptions::AllExpired => fetch_reminders(&ctx.data().database).await?,
    };

    let mut response = String::new();
    let mut index = 0;
    for reminder in reminders {
        if reminder.active {
            index += 1;
            let time_left = format_duration((reminder.end_time - Utc::now()).to_std().unwrap());
            let user_id = users::get_user_from_db_id(&ctx.data().database, &reminder.user_id)
                .await?
                .user_id;

            let user = UserId(user_id as u64).to_user(ctx.http()).await?;

            response += &format!(
                "{index}. {} -- {} -- time left - {}\n",
                user.name, reminder.description, time_left
            )
        }
    }
    ctx.send(|m| m.embed(|e| e.title("Reminders").description(response)))
        .await?;
    Ok(())
}
