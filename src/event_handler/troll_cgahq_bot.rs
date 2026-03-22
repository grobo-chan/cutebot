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
            .reply(&ctx.http, "only if you open source")
            .await?;
        new_message.channel_id.say(&ctx.http, "https://tenor.com/view/girls-last-tour-shoujo-shuumatsu-ryokou-squish-stretch-book-gif-20324471").await?;
    } else if msg_has_keywords(&new_message.content, vec!["my creators are lazy fuck"]).await? {
        sleep(Duration::from_secs(4)).await;
        new_message
            .reply(&ctx.http, "https://git-scm.com/learn")
            .await?;
        new_message
            .channel_id
            .say(&ctx.http, "now go open source you noob")
            .await?;
    }

    Ok(())
}
