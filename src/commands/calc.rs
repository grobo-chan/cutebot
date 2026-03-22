/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::utils::processor::processor;
use crate::{Context, Error};

use poise::serenity_prelude as serenity;
use serenity::all::CreateEmbedAuthor;
use serenity::builder::CreateEmbed;

/// Literally just a calculator with a dice function
#[poise::command(slash_command, prefix_command)]
pub async fn calc(ctx: Context<'_>, expr: String) -> Result<(), Error> {
    let result = processor(expr.as_str());

    let embed_author =
        CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name())).icon_url(
            ctx.author()
                .avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url()),
        );

    let embed = CreateEmbed::new()
        .author(embed_author)
        .colour(serenity::Colour::DARK_MAGENTA)
        .title("Calculation Complete!")
        .description(format!("The result is: {}", result.await));

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
