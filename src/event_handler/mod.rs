/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

mod chatbot;
mod landmine;
mod troll_cgahq_bot;
mod update_leaderboard;

use crate::event_handler::chatbot::chatbot;
use crate::event_handler::landmine::landmine;
use crate::event_handler::troll_cgahq_bot::troll_cgahq_bot;
use crate::event_handler::update_leaderboard::update_channel;
use crate::sql::add_new_member::add_new_member;
use crate::sql::reset_server::reset_server;
use crate::sql::settings::get_setting;
use crate::{Data, Error};

use poise::serenity_prelude as serenity;
use rand::prelude::*;

const CGAHQ_BOT_ID: u64 = 1468954832764276856;

async fn msg_has_keywords(msg: &String, keywords: Vec<&str>) -> Result<bool, Error> {
    Ok(keywords.iter().any(|&x| {
        msg.to_lowercase()
            .as_str()
            .contains(x.to_lowercase().as_str())
    }))
}

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
            if new_message.author.id.get() == CGAHQ_BOT_ID {
                troll_cgahq_bot(&new_message, &ctx).await?;
            }

            match ctx.http.get_current_application_info().await {
                Ok(info) => {
                    if let Some(guild_id) = new_message.guild_id {
                        if new_message.application_id == Some(info.id) {
                            update_channel(guild_id, ctx, data).await?;
                        }
                    }
                }
                Err(e) => panic!("Error while fetching app info: {:?}", e),
            }

            if new_message.author.bot {
                return Ok(());
            }

            if let Some(guild_id) = new_message.guild_id {
                let server_landmine: u64 = get_setting(guild_id, "landmine_channel", data)
                    .await?
                    .parse()
                    .unwrap_or_default();

                let landmine_immune_role: u64 =
                    get_setting(guild_id, "landmine_immunity_role", data)
                        .await?
                        .parse()
                        .unwrap_or_default();

                if (new_message.channel_id.get() == server_landmine)
                    && (num == 1)
                    && !(new_message
                        .author
                        .has_role(&ctx.http, guild_id, landmine_immune_role)
                        .await?)
                {
                    landmine(new_message, ctx).await?;
                }

                if (new_message.mentions_me(&ctx.http).await?)
                    || (msg_has_keywords(&new_message.content, vec!["cutebot", "cute bot"]).await?)
                {
                    chatbot(new_message, ctx).await?;
                }
            }
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            add_new_member(new_member, data).await?;
        }
        serenity::FullEvent::GuildCreate { guild, is_new } => {
            if is_new.unwrap_or(false) {
                reset_server(&guild.id, data, &ctx.http).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
