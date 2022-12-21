use crate::{db, Context, Error};
use chrono::prelude::*;
use poise::serenity_prelude as serenity;
#[poise::command(slash_command)]
pub async fn create_entry(
    ctx: Context<'_>,
    #[description = "time you want the timer to run"] time: i64,
    #[description = "people you want to complete the entry"] users: serenity::User,
) -> Result<(), Error> {
    let current_time = Utc::now();
    let duration = chrono::Duration::hours(time);
    let entry = db::Entry {
        start_time: current_time,
        end_time: current_time + duration,
    };
    let member = db::Member {
        name: users.name,
        user_id: users.id.into(),
    };

    //we want to make sure that this thread is the only one with access to the db
    let mut conn = ctx.data().database.acquire().await?;

    let _insert_members = sqlx::query(
        "INSERT INTO members ( user_id,  name)
          VALUES (?, ?);",
    )
    .bind(member.user_id)
    .bind(member.name)
    .execute(&mut conn)
    .await?;

    let _inset_entry = sqlx::query(
        "INSERT INTO entries ( start_time, end_time)
           VALUES (?,?);",
    )
    .bind(entry.start_time)
    .bind(entry.end_time)
    .execute(&mut conn)
    .await?;

    //searching
    let search_db = sqlx::query_as::<_, db::Member>("SELECT user_id, name FROM members")
        .fetch_all(&mut conn)
        .await?;

    for user in search_db {
        println!("{} {}", user.user_id, user.name);
    }

    let entry_search = sqlx::query_as::<_, db::Entry>("SELECT start_time, end_time FROM entries")
        .fetch_all(&mut conn)
        .await?;

    for entry in entry_search {
        println!("{} {}", entry.start_time, entry.end_time);
    }

    let response = format!("trust the backend");
    ctx.say(response).await?;
    Ok(())
}
