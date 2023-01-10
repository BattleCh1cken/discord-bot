use crate::{db, Context, Error};

use poise::serenity_prelude::{self as serenity, CacheHttp};

/// Boop the bot!
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn boop(ctx: Context<'_>) -> Result<(), Error> {
    let uuid_boop = ctx.id();
    //query the db, look for existing score
    let mut boop_count =
        db::boop::search_for_score(&ctx.data().database, ctx.author().into()).await?;
    println!("{boop_count}");

    ctx.send(|m| {
        m.content(format!("current score: {}", boop_count))
            .components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(serenity::ButtonStyle::Primary)
                            .label("Boop me!")
                            .custom_id(uuid_boop)
                    })
                })
            })
    })
    .await?;

    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == uuid_boop.to_string())
        .await
    {
        //Retrieve existing score
        boop_count = db::boop::search_for_score(&ctx.data().database, ctx.author().into()).await?;
        boop_count += 1;
        //Update score to new score
        db::boop::update_score(&ctx.data().database, boop_count, ctx.author().into()).await?;

        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| m.content(format!("Boop count: {boop_count}")))
            .await?;

        mci.create_interaction_response(ctx, |ir| {
            ir.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let scores = db::boop::get_top_scores(&ctx.data().database).await?;
    println!("{:#?}", scores);
    let mut response = String::new();
    let mut index = 0;
    for score in scores {
        let user = serenity::UserId(score.user_id as u64)
            .to_user(&ctx.http())
            .await?;

        index += 1;
        response += &format!("{}. {} -- {}", index, user.name, score.score);
    }

    //Why I can't use ctx.say() here I have no idea
    poise::send_reply(ctx, |f: &mut poise::CreateReply<'_>| {
        f.embed(|f| f.title("Top Boop Counts:").description(response))
    })
    .await?;

    Ok(())
}
