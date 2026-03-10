use crate::Error;
use crate::utils::fetch_emote::fetch_emote;

use poise::serenity_prelude as serenity;
use serenity::all::{CreateMessage, Mentionable};

pub async fn chatbot(
    new_message: &serenity::all::Message,
    ctx: &serenity::Context,
) -> Result<(), Error> {
    let has_rel_keywords = vec![
        "you in a relationship",
        "u in a relationship",
        "are you dating",
        "r u dating",
    ]
    .iter()
    .any(|&x| new_message.content.to_lowercase().as_str().contains(x));

    let asked_pronouns = vec!["what are your pronouns", "what r ur pronouns"]
        .iter()
        .any(|&x| new_message.content.to_lowercase().as_str().contains(x));

    if has_rel_keywords {
        let cgahq_bot = serenity::UserId::new(1468954832764276856);
        let guild_id = new_message.guild_id.ok_or("Not in a guild")?;

        match guild_id.member(&ctx.http, cgahq_bot).await {
            Ok(_) => {
                let aunn_blush_emoji = fetch_emote(&ctx.http, "aunnblush".to_string()).await?;

                new_message
                    .reply_ping(
                        &ctx.http,
                        format!("I have a crush on {}...", cgahq_bot.mention()),
                    )
                    .await?;
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
    } else if asked_pronouns {
        let letsu_emote = fetch_emote(&ctx.http, "letsu".to_string()).await?;
        new_message.reply(&ctx.http, "i go by it/its").await?;
        new_message
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().content(format!("{}", letsu_emote)),
            )
            .await?;
    }

    Ok(())
}
