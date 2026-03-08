mod landmine;
mod member_join;

use crate::event_handler::landmine::landmine;
use crate::event_handler::member_join::member_join;
use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use rand::prelude::*;
use serenity::builder::EditChannel;

const DESC: &str = "This text channel HAS A BILLION LANDMINES YOU WILL EXPLODE.\n (1/6 chance of a 10min timeout unless you mod.)";
const COURT_CHANNEL_ID: serenity::ChannelId = serenity::ChannelId::new(1450186078249291866);
const POS_ANNOYING_CHANNEL_ID: serenity::ChannelId = serenity::ChannelId::new(942627172705779863);

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

            COURT_CHANNEL_ID
                .edit(ctx, EditChannel::new().topic(DESC))
                .await?;
        }
        serenity::FullEvent::Message { new_message } => {
            if !new_message.author.bot
                && (new_message.channel_id == COURT_CHANNEL_ID
                    || new_message.channel_id == POS_ANNOYING_CHANNEL_ID)
                && num == 1
            {
                landmine(new_message, ctx).await?;
            }
        }
        serenity::FullEvent::ChannelUpdate { old: _, new } => {
            if new.id == COURT_CHANNEL_ID {
                COURT_CHANNEL_ID
                    .edit(ctx, EditChannel::new().topic(DESC))
                    .await?;
            }
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            member_join(new_member, data).await?;
        }
        _ => {}
    }
    Ok(())
}
