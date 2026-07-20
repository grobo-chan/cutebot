/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::sql::settings::get_all_settings;
use crate::{Context, Error};

use poise::serenity_prelude as serenity;

/// Gets value of a setting
#[poise::command(slash_command, prefix_command)]
pub async fn get_all(ctx: Context<'_>) -> Result<(), Error> {
    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );
    let server_id = ctx.guild_id().expect("Not in a server!");
    let values = get_all_settings(server_id, &ctx.data()).await?;

    // TODO: Make it prettier
    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .description(format!(
            "**Settings:**\n {}",
            values
                .iter()
                .map(|x| { format!("- {} {}", x[0], x[1]) })
                .collect::<Vec<_>>()
                .join("\n")
        ));

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
