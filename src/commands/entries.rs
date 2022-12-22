use crate::{Context, Error};
use chrono::prelude::*;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn create_entry(
    ctx: Context<'_>,
    #[description = "time you want the timer to run"] time: i64,
    #[description = "people you want to complete the entry"] users: serenity::User,
) -> Result<(), Error> {
    let current_time = Utc::now();
    let duration = chrono::Duration::hours(time);
    let start_time = current_time;
    let end_time = current_time + duration;

    let name = users.name;
    let id: i64 = users.id.into();

    //we want to make sure that this thread is the only one with access to the db
    let mut conn = ctx.data().database.acquire().await?;
    let entry_id = sqlx::query!(
        "insert into entries (start_time, end_time) values(?, ?)",
        start_time,
        end_time
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();
    let member_id = sqlx::query!("insert into members (user_id, name) values(?, ?)", name, id,)
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

    let _le_binding = sqlx::query!(
        "insert into member_entries(entry_id, member_id) values(?,?)",
        entry_id,
        member_id
    )
    .execute(&mut conn)
    .await?;

    let response = format!("Created entry for user '{}'", name);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn list_entries(ctx: Context<'_>) -> Result<(), Error> {
    let mut conn = ctx.data().database.acquire().await?;

    let search = sqlx::query!(
        // just pretend "accounts" is a real table
        "select id,entry_id, member_id from member_entries"
    )
    .fetch_all(&mut conn)
    .await?;
    for entry in search {
        let response = format!(
            "Id: {:?}  Entry id: {:?}  Member id:{:?}",
            entry.id,
            entry.entry_id.unwrap(),
            entry.member_id.unwrap()
        );
        ctx.say(response).await?;
    }

    Ok(())
}
