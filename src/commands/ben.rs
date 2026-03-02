use crate::{Context, Error};
use poise::serenity_prelude as serenity;

use ::serenity::all::Mentionable;
use serenity::all::CreateEmbedAuthor;
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

    let app_emojis = ctx.http().get_application_emojis().await?;
    let loading_emoji = app_emojis
        .iter()
        .find(|e| e.name == "loading")
        .map(|e| {
            if e.animated {
                format!("<a:{}:{}>", e.name, e.id)
            } else {
                format!("<:{}:{}>", e.name, e.id)
            }
        })
        .unwrap_or_else(|| ":loading:".to_string());

    let embed = CreateEmbed::new().author(embed_author).description(format!(
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
