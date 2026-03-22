/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::Error;
use crate::event_handler::msg_has_keywords;

use poise::serenity_prelude as serenity;
use tokio::time::{Duration, sleep};

pub async fn troll_cgahq_bot(
    new_message: &serenity::all::Message,
    ctx: &serenity::Context,
) -> Result<(), Error> {
    if msg_has_keywords(
        &new_message.content,
        vec!["obviously im literally the best bot ever made, trust"],
    )
    .await?
    {
        new_message.reply(&ctx.http, "no me").await?;
    } else if msg_has_keywords(&new_message.content, vec!["no both"]).await? {
        sleep(Duration::from_secs(4)).await;
        new_message
            .reply(
                &ctx.http,
                "only if you open source... oh wait you're open source now based",
            )
            .await?;
        new_message.channel_id.say(&ctx.http, "https://tenor.com/view/girls-last-tour-shoujo-shuumatsu-ryokou-squish-stretch-book-gif-20324471").await?;
    } else if msg_has_keywords(&new_message.content, vec!["bad code"]).await? {
        sleep(Duration::from_secs(4)).await;
        new_message
            .reply(&ctx.http, "mine is linked in my bio uwu")
            .await?;
    }

    Ok(())
}
