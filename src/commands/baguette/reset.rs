/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::sql::reset_server::reset_server;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};

/// Initiliazes/Reset the economy by setting everyone at 100 baguettes
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    aliases("init")
)]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    reset_server(
        ctx.guild_id().get_or_insert(serenity::GuildId::new(1)),
        &ctx.data(),
        ctx.http(),
    )
    .await?;

    let embed_author =
        CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name())).icon_url(
            ctx.author()
                .avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url()),
        );

    let embed = CreateEmbed::new()
        .title("Economy successfully reset")
        .colour(serenity::Colour::DARK_GREEN)
        .author(embed_author)
        .description("The Baguette economy has now been reset.");

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
