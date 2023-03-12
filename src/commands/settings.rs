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

    let reminder_channel = reminder_channel.map(|id| *id.guild().unwrap().id.as_u64() as i64);
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

    let guild_name = guild.name(ctx).unwrap();

    let reminder_channel_name = match new_guild_settings.reminder_channel {
        Some(id) => {
            ChannelId(id as u64)
                .to_channel_cached(&ctx)
                .unwrap()
                .guild()
                .unwrap()
                .name
        }
        None => "None".to_string(),
    };

    let reminder_master_role_name = match new_guild_settings.reminder_master_role {
        Some(id) => {
            RoleId(id as u64).to_role_cached(ctx).unwrap().name
        }
        None => "None".to_string(),
    };

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Settings").description(format!(
                "
                **Guild:** {}
                **Reminder Master Role:** {}
                **Reminder Channel:** {}
                ",
                guild_name, reminder_master_role_name, reminder_channel_name,
            ))
        })
    })
    .await?;

    Ok(())
}
