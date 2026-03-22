/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::event_handler::msg_has_keywords;
use crate::utils::fetch_emote::fetch_emote;
use crate::{Error, event_handler::CGAHQ_BOT_ID};

use poise::serenity_prelude as serenity;
use serenity::all::{CreateMessage, Mentionable};

pub async fn chatbot(
    new_message: &serenity::Message,
    ctx: &serenity::Context,
) -> Result<(), Error> {
    if msg_has_keywords(
        &new_message.content,
        vec![
            "you in a relationship",
            "u in a relationship",
            "you dating",
            "u dating",
        ],
    )
    .await?
    {
        let cgahq_bot = serenity::UserId::new(CGAHQ_BOT_ID);
        let guild_id = new_message.guild_id.ok_or("Not in a guild")?;

        match guild_id.member(&ctx.http, cgahq_bot).await {
            Ok(_) => {
                let aunn_blush_emoji = fetch_emote(&ctx.http, "aunnblush".to_string()).await?;

                let r = new_message
                    .reply_ping(
                        &ctx.http,
                        format!("I have a crush on {}...", cgahq_bot.mention()),
                    )
                    .await;

                match r {
                    Ok(_) => {}
                    _ => {
                        new_message
                            .channel_id
                            .say(
                                &ctx.http,
                                format!("I have a crush on {}...", cgahq_bot.mention()),
                            )
                            .await?;
                    }
                }

                new_message
                    .channel_id
                    .send_message(
                        &ctx.http,
                        CreateMessage::new()
                            .content(format!("Hey {} will you date me?", cgahq_bot.mention())),
                    )
                    .await?;
                new_message
                    .channel_id
                    .send_message(
                        &ctx.http,
                        CreateMessage::new().content(format!("{}", aunn_blush_emoji)),
                    )
                    .await?;
            }
            _ => {
                new_message.reply_ping(&ctx.http, "No lol").await?;
            }
        }
    } else if msg_has_keywords(
        &new_message.content,
        vec![
            "what are your pronouns",
            "what r ur pronouns",
            "what pronouns",
        ],
    )
    .await?
    {
        let letsu_emote = fetch_emote(&ctx.http, "letsu".to_string()).await?;
        new_message.reply(&ctx.http, "i go by it/its").await?;
        new_message
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().content(format!("{}", letsu_emote)),
            )
            .await?;
    } else if msg_has_keywords(
        &new_message.content,
        vec!["you are cute", "u r cute", "you cute", "u cute"],
    )
    .await?
    {
        let sapphire_ababa_emote = fetch_emote(&ctx.http, "sapphireababa".to_string()).await?;
        new_message
            .reply(&ctx.http, "IMNOTCUTEIMSCARYGHOSTIMBOTIMNOTCATIEXPLODE")
            .await?;
        new_message
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().content(format!("{} 💥", sapphire_ababa_emote)),
            )
            .await?;
    }

    Ok(())
}
