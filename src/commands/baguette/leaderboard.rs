/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::sql::get_baguettes_data::get_all_baguettes_data;
use crate::utils::paginate::{get_first_page_embed, get_pages, paginate_embed_message};
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// The baguette leaderboard
#[poise::command(slash_command, prefix_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    if let Some(guild_id) = ctx.guild_id() {
        let info = get_all_baguettes_data(guild_id, &ctx.data()).await?;
        let pages = get_pages(info).await?;
        let (embed, components) =
            get_first_page_embed(&pages, Some(author.clone()), ctx.id()).await?;

        let reply = poise::CreateReply::default()
            .embed(embed)
            .components(vec![components]);
        ctx.send(reply).await?;

        paginate_embed_message(
            &ctx.serenity_context(),
            &pages,
            Some(author.clone()),
            true,
            ctx.id(),
        )
        .await?;
    }

    Ok(())
}
