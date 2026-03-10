mod chatbot;
mod landmine;
mod member_join;

use crate::event_handler::chatbot::chatbot;
use crate::event_handler::landmine::landmine;
use crate::event_handler::member_join::member_join;
use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use rand::prelude::*;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    let num = rand::rng().random_range(1..=6);
    // let num = 1;

    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }

            let is_landmine_channel = vec![1450186078249291866, 942627172705779863]
                .iter()
                .any(|&x| new_message.channel_id == serenity::ChannelId::new(x));

            if is_landmine_channel && num == 1 {
                landmine(new_message, ctx).await?;
            }

            if new_message.mentions_me(&ctx.http).await? {
                chatbot(new_message, ctx).await?;
            }
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            member_join(new_member, data).await?;
        }
        _ => {}
    }
    Ok(())
}
