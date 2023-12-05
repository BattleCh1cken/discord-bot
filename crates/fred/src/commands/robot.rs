use crate::{Context, Error};

#[poise::command(slash_command, subcommands("team"))]
pub async fn vex(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn team(ctx: Context<'_>, team: String) -> Result<(), Error> {
    let team = &ctx.data().robotevents.get_team(&team).await?[0];
    ctx.send(|m| {
        m.embed(|e| {
            e.title(&team.number)
                .field("Program", &team.program.code, true)
                .field("Grade", &team.grade, true)
                .field("Active", &team.registered, true)
                .field(
                    "Location",
                    format!("{}, {}", &team.location.city, &team.location.region),
                    false,
                )
        })
    })
    .await?;

    Ok(())
}
