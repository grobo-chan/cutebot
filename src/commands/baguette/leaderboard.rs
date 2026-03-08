use crate::{Context, Error};

use futures::{StreamExt, TryStreamExt};
use poise::serenity_prelude as serenity;
use serenity::all::{ChannelId, CreateMessage, Mentionable, UserId};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};

pub async fn get_embed(ctx: &Context<'_>) -> Result<CreateEmbed, Error> {
    let mut rows =
        sqlx::query!("SELECT * FROM balance ORDER BY money desc;").fetch(&ctx.data().database);

    let mut desc = String::new();
    let mut i = 1;
    while let Some(row) = rows.try_next().await? {
        let id: UserId = UserId::new(row.user_id as u64);
        desc.push_str(format!("{}. {} {}\n", i, id.mention(), row.money).as_str());
        i += 1;
    }

    let embed_author =
        CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name())).icon_url(
            ctx.author()
                .avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url()),
        );

    let embed = CreateEmbed::new()
        .title("Leaderboard")
        .colour(serenity::Colour::DARK_GREEN)
        .author(embed_author)
        .description(desc);

    Ok(embed)
}

pub async fn update_channel(ctx: &Context<'_>) -> Result<(), Error> {
    let leaderboard_channel = ChannelId::new(1480154675608027226);
    let http = ctx.http();
    let mut messages = leaderboard_channel.messages_iter(&http).boxed();

    while let Some(msg) = messages.next().await {
        match msg {
            Ok(m) => m.delete(&ctx.http()).await?,
            Err(e) => eprintln!("Error deleting message: {}", e),
        }
    }

    let embed = get_embed(&ctx).await?;
    let message = CreateMessage::new().embed(embed);

    leaderboard_channel.send_message(http, message).await?;

    Ok(())
}

/// The baguette leaderboard
#[poise::command(slash_command, prefix_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let embed = get_embed(&ctx).await?;
    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    update_channel(&ctx).await?;
    Ok(())
}
