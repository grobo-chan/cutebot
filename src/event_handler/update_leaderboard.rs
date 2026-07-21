/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::sql::get_baguettes_data::get_all_baguettes_data;
use crate::sql::settings::{edit_setting, get_setting};
use crate::utils::paginate::{get_first_page_embed, get_pages, paginate_embed_message};
use crate::{Data, Error};
use futures::StreamExt;
use poise::serenity_prelude as serenity;
use serenity::CacheHttp;

pub async fn update_channel(
    guild_id: serenity::GuildId,
    ctx: &serenity::Context,
    data: &Data,
) -> Result<(), Error> {
    let channel_id: u64 = get_setting(guild_id, "leaderboard_channel", data)
        .await?
        .parse()
        .unwrap_or_default();
    let mut message_id: u64 = get_setting(guild_id, "leaderboard_message", data)
        .await?
        .parse()
        .unwrap_or_default();

    if channel_id != 0 {
        let leaderboard_channel = serenity::ChannelId::new(channel_id);
        let http = ctx.http();

        if message_id == 0 {
            let msg = leaderboard_channel
                .say(&http, "Generating Leaderboard...")
                .await?;
            message_id = msg.id.get();
            edit_setting(guild_id, "leaderboard_message", message_id, data).await?;
        }

        let mut messages = leaderboard_channel.messages_iter(&http).boxed();

        let mut leaderboard_msg = leaderboard_channel.message(&ctx.http, message_id).await?;
        while let Some(msg) = messages.next().await {
            match msg {
                Ok(m) => {
                    if m.id.get() != message_id {
                        m.delete(&ctx.http()).await?;
                    }
                }
                Err(e) => eprintln!("Error deleting message: {}", e),
            }
        }

        let info = get_all_baguettes_data(guild_id, data).await?;
        let pages = get_pages(info).await?;
        let (embed, components) = get_first_page_embed(&pages, None, channel_id).await?;

        let msg_edit = serenity::EditMessage::new()
            .content("")
            .embed(embed)
            .components(vec![components]);
        leaderboard_msg.edit(&ctx.http, msg_edit).await?;

        paginate_embed_message(&ctx, &pages, None, false, channel_id).await?;
    }

    Ok(())
}
