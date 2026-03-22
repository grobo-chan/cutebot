/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::utils::fetch_emote::fetch_emote;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

use serenity::all::{CreateEmbedAuthor, Mentionable};
use serenity::builder::CreateEmbed;

/// Bens one or more users
#[poise::command(slash_command, prefix_command)]
pub async fn ben(
    ctx: Context<'_>,
    #[description = "User to ben"] users: Vec<serenity::User>,
) -> Result<(), Error> {
    let embed_author =
        CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name())).icon_url(
            ctx.author()
                .avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url()),
        );

    let loading_emoji = fetch_emote(&ctx.http(), "loading".to_string()).await?;

    let embed = CreateEmbed::new()
        .author(embed_author)
        .colour(serenity::Colour::DARK_GREEN)
        .description(format!(
            "**Benning:**\n {}",
            users
                .iter()
                .map(|x| { format!("- {} {}", x.mention(), loading_emoji) })
                .collect::<Vec<_>>()
                .join("\n")
        ));

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
