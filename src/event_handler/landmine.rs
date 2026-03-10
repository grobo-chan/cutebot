use crate::Error;

use futures::StreamExt;
use poise::serenity_prelude as serenity;
use serenity::all::{Mentionable, ReactionType};
use serenity::builder::{CreateButton, CreateEmbed, CreateMessage};
use std::time::Duration;

pub async fn landmine(
    new_message: &serenity::all::Message,
    ctx: &serenity::Context,
) -> Result<(), Error> {
    let seconds = 600; // 10 minute timeout

    let until = serenity::Timestamp::from_unix_timestamp(
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
            + seconds) as i64,
    )?;

    let guild_id = new_message.guild_id.ok_or("Not in a guild")?;
    let mut member = guild_id.member(ctx, new_message.author.id).await?;

    let result = member
        .edit(
            ctx,
            serenity::EditMember::new().disable_communication_until_datetime(until),
        )
        .await;

    let embed = match result {
        Ok(_) => CreateEmbed::new()
            .title("A Landmine has exploded!")
            .colour(serenity::Colour::RED)
            .description(format!(
                "{} has been timed out for 10 minutes",
                new_message.author.mention()
            )),
        Err(e) => CreateEmbed::new()
            .title("A Landmine failed to explode")
            .colour(serenity::Colour::LIGHT_GREY)
            .description(format!(
                "{} is safe... probably because {}",
                new_message.author.mention(),
                e
            )),
    };

    let builder = CreateMessage::new()
        .embed(embed)
        .button(CreateButton::new("Delete").emoji("🗑️".parse::<ReactionType>().unwrap()));

    let m = new_message.channel_id.send_message(ctx, builder).await?;

    let mut interaction_stream = m
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .stream();

    while let Some(_interaction) = interaction_stream.next().await {
        m.delete(&ctx).await.unwrap();
    }

    Ok(())
}
