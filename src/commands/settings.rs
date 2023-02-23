use poise::serenity_prelude::{Channel, ChannelId, Role, RoleId};

use crate::{
    commands::checks::is_administrator,
    db::guilds::{create_guild, get_guild, update_guild_settings, Guild},
    Context, Error,
};

#[poise::command(slash_command, guild_only, check = "is_administrator")]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "The role that has the power to set reminders for other users"]
    reminder_master_role: Option<Role>,
    #[description = "The channel in which reminders will be sent"] reminder_channel: Option<
        Channel,
    >,
) -> Result<(), Error> {
    //Create guild in db if not exists
    let guild = ctx.guild_id().unwrap();

    // TODO: find a better way to do this
    let reminder_channel = reminder_channel.map(|id| *id.id().as_u64() as i64);
    let reminder_master_role = reminder_master_role.map(|role| *role.id.as_u64() as i64);

    create_guild(&ctx.data().database, &guild).await?;
    let old_guild_settings = get_guild(&ctx.data().database, &guild).await?;
    let new_guild_settings = Guild {
        id: 0,
        guild_id: *guild.as_u64() as i64,
        reminder_master_role,
        reminder_channel,
    }
    .merge(&old_guild_settings);

    update_guild_settings(&ctx.data().database, &new_guild_settings).await?;

    let reminder_channel_name =
        ChannelId(new_guild_settings.reminder_master_role.unwrap_or(0) as u64)
            .to_channel(ctx)
            .await?
            .guild()
            .unwrap()
            .name;

    let reminder_master_role_name =
        RoleId(new_guild_settings.reminder_master_role.unwrap_or(0) as u64)
            .to_role_cached(ctx)
            .unwrap()
            .name;

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Settings").description(format!(
                "
                **Guild:** {}
                **Reminder Master Role:** {}
                **Reminder Channel:** {}
                ",
                new_guild_settings.guild_id, reminder_master_role_name, reminder_channel_name,
            ))
        })
    })
    .await?;

    Ok(())
}
