use crate::{Context, Error};

use futures::{StreamExt, TryStreamExt};
use poise::serenity_prelude as serenity;
use serenity::all::{ChannelId, CreateMessage, Mentionable, UserId};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use sqlx::{QueryBuilder, Row, Sqlite};

pub async fn get_embed(ctx: &Context<'_>) -> Result<CreateEmbed, Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut rows_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT user_id, baguettes FROM balance WHERE server_id = {} ORDER BY baguettes desc;",
        guild_id.get()
    ));

    let mut rows = rows_query_builder.build().fetch(&ctx.data().database);

    let mut desc = String::new();
    let mut i = 1;
    while let Some(row) = rows.try_next().await? {
        let id: u64 = row.try_get("user_id")?;
        let baguettes: u64 = row.try_get("baguettes")?;
        desc.push_str(format!("{}. {} {}\n", i, UserId::new(id).mention(), baguettes).as_str());
        i += 1;
    }

    let embed = CreateEmbed::new()
        .title("Leaderboard")
        .colour(serenity::Colour::DARK_GREEN)
        .description(desc);

    Ok(embed)
}

pub async fn update_channel(ctx: &Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut id_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT leaderboard_channel FROM servers WHERE server_id = {};",
        guild_id
    ));

    let channel_id: u64 = id_query_builder
        .build()
        .fetch_one(&ctx.data().database)
        .await?
        .try_get("leaderboard_channel")?;

    if channel_id != 0 {
        let leaderboard_channel = ChannelId::new(channel_id);
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
    }

    Ok(())
}

/// The baguette leaderboard
#[poise::command(slash_command, prefix_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let mut embed = get_embed(&ctx).await?;

    let embed_author =
        CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name())).icon_url(
            ctx.author()
                .avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url()),
        );
    embed = embed.author(embed_author);

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    update_channel(&ctx).await?;
    Ok(())
}
