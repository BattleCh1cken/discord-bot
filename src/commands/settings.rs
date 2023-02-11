use poise::serenity_prelude::{Channel, Role};

use crate::{
    db::guilds::{create_guild, get_guild, Guild},
    Context, Error,
};

#[poise::command(slash_command, guild_only)]
pub async fn settings(
    ctx: Context<'_>,
    reminder_role: Option<Role>,
    reminder_channel: Option<Channel>,
) -> Result<(), Error> {
    //Create guild in db if not exists
    let guild = ctx.guild_id().unwrap();

    let reminder_channel = reminder_channel.map(|id| *id.id().as_u64() as i64);
    let reminder_master_role = reminder_role.map(|role| *role.id.as_u64() as i64);

    create_guild(&ctx.data().database, &guild).await?;
    let guild_old = get_guild(&ctx.data().database, &guild).await?;
    let guild_new = Guild {
        id: 0,
        guild_id: guild.as_u64().clone() as i64,
        reminder_master_role,
        reminder_channel,
    }
    .merge(guild_old);

    

    ctx.say(format!("{:#?}", guild_new)).await?;

    Ok(())
}
