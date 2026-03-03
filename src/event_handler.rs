use crate::{Data, Error};
use ::serenity::all::{CreateMessage, Mentionable};
use poise::serenity_prelude as serenity;
use serenity::builder::CreateEmbed;

use rand::RngExt;
use std::env;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    let court_channel = serenity::ChannelId::new(1450186078249291866);
    let num = rand::rng().random_range(1..=20);

    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name)
        }
        serenity::FullEvent::Message { new_message } => {
            if !new_message.author.bot && new_message.channel_id == court_channel && num == 1 {
                let seconds = 600; // 10 minute timeout

                let until = serenity::Timestamp::from_unix_timestamp(
                    (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs()
                        + seconds) as i64,
                )?;

                let guild_id_str = env::var("GUILD_ID").expect("missing GUILD_ID");
                let guild_id_num = guild_id_str.parse::<u64>().expect("invalid GUILD_ID");
                let guild_id = serenity::GuildId::new(guild_id_num);
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

                let builder = CreateMessage::new().embed(embed);
                new_message.channel_id.send_message(ctx, builder).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
