/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::utils::parse_emoji;
use serenity::{CreateEmbed, CreateEmbedAuthor, EmojiIdentifier};

/// Gets emotes
#[poise::command(slash_command, prefix_command)]
pub async fn getemote(ctx: Context<'_>, emotes: String) -> Result<(), Error> {
    let emotes_list = emotes
        .split_ascii_whitespace()
        .filter_map(|s| parse_emoji(s))
        .collect::<Vec<EmojiIdentifier>>();

    let embed_author =
        CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name())).icon_url(
            ctx.author()
                .avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url()),
        );

    let embed = CreateEmbed::new()
        .author(embed_author)
        .colour(serenity::Colour::DARK_GREEN)
        .description(format!(
            "**The following emotes were fetched:**\n {}",
            emotes_list
                .iter()
                .map(|e| { format!("- `:{}:`", e.name) })
                .collect::<Vec<_>>()
                .join("\n")
        ));

    let mut reply = poise::CreateReply::default().embed(embed);
    for e in emotes_list {
        reply = reply.clone().attachment(
            serenity::CreateAttachment::url(&ctx.http(), e.url().as_str())
                .await
                .unwrap(),
        );
    }

    ctx.send(reply).await?;

    Ok(())
}
