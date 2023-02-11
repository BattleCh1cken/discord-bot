use crate::{
    db::{self, entries::*, users},
    Context, Error,
};
use chrono::prelude::*;
use humantime::{self, format_duration};
use poise::serenity_prelude::{self as serenity, CacheHttp, Mentionable, UserId};

async fn check_if_is_notebooker(ctx: Context<'_>) -> Result<bool, Error> {
    let result = ctx
        .author()
        .has_role(
            ctx.http(),
            *ctx.data().guild_id,
            *ctx.data().notebooker_role,
        )
        .await?;

    if !result {
        ctx.say("You aren't a notebooker").await?;
    }
    Ok(result)
}

///Commands that handle notebook entries
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("create", "list", "complete")
)]
pub async fn entry(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

///create a new entry timer
#[poise::command(slash_command, prefix_command, check = "check_if_is_notebooker")]
pub async fn create(
    ctx: Context<'_>,
    #[description = "time you want the timer to run, in hours"] time: i64,
    #[description = "person you want to complete the entry"] user: serenity::User,
    #[description = "what you want them to write in that entry"] description: String,
    #[description = "whether you want the user to be reminded before the entry expires"]
    remind: Option<bool>,
) -> Result<(), Error> {
    let current_time = Utc::now();
    //Change this to hours
    let duration = chrono::Duration::hours(time);
    let end_time = current_time + duration;

    let remind = remind.unwrap_or_else(|| false);
    db::users::create_user(&ctx.data().database, &user.clone().into()).await?;

    let user_id = db::users::get_user_from_id(&ctx.data().database, &user.clone().into()).await?;

    db::entries::insert_entry(
        &ctx.data().database,
        &end_time,
        &user_id.id,
        &description,
        &remind,
    )
    .await?;

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Entry timer created").description(format!(
                "
                **User:** {}
                **Time to complete:** {} hours
                **Description:** {}
                **Remind:** {}
                ",
                user.mention(),
                time,
                description,
                remind
            ))
        })
    })
    .await?;
    Ok(())
}

///Marks your entries as complete, absolves you of shame
#[poise::command(slash_command, prefix_command)]
pub async fn complete(ctx: Context<'_>) -> Result<(), Error> {
    let user: UserId = ctx.author().into();
    db::entries::complete_entry(&ctx.data().database, user).await?;

    ctx.say("Entries marked as complete").await?;

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
//Displays entries
#[poise::command(slash_command, prefix_command)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "whether to display entries that are expired"] view: Option<ViewOptions>,
) -> Result<(), Error> {
    let user = ctx.author().id;

    let entries = match view.unwrap_or_else(|| ViewOptions::Author) {
        ViewOptions::Author => {
            let db_user_id = db::users::get_user_from_id(&ctx.data().database, &user)
                .await?
                .id;
            fetch_active_entries_for_user(&ctx.data().database, &db_user_id).await?
        }
        ViewOptions::AuthorExpired => {
            let db_user_id = db::users::get_user_from_id(&ctx.data().database, &user)
                .await?
                .id;
            fetch_entries_for_user(&ctx.data().database, &db_user_id).await?
        }
        ViewOptions::All => fetch_active_entries(&ctx.data().database).await?,

        ViewOptions::AllExpired => fetch_entries(&ctx.data().database).await?,
    };

    let mut response = String::new();
    let mut index = 0;
    for entry in entries {
        if entry.active {
            index += 1;
            let time_left = format_duration((entry.end_time - Utc::now()).to_std().unwrap());
            let user_id = users::get_user_from_db_id(&ctx.data().database, &entry.user_id)
                .await?
                .user_id;

            let user = UserId(user_id.try_into().unwrap())
                .to_user(ctx.http())
                .await?;

            response += &format!(
                "{index}. {} -- {} -- time left - {}\n",
                user.name, entry.description, time_left
            )
        }
    }
    ctx.send(|m| m.embed(|e| e.title("Entries due").description(response)))
        .await?;
    Ok(())
}
