// use crate::commands::baguette::leaderboard::update_channel;
use crate::{Context, Error};

use ::serenity::all::Member;
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use sqlx::query_builder::QueryBuilder;
use sqlx::{Execute, Sqlite};

/// Initiliazes/Reset the economy by setting everyone at 100 baguettes
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    aliases("init")
)]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;

    let mut settings_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "INSERT OR IGNORE INTO servers (server_id, leaderboard_channel, gambling_enabled) VALUES ({}, NULL, 0);",
        guild_id.get()
    ));

    let settings_query = settings_query_builder.build();
    println!("{}", settings_query.sql());
    settings_query.execute(&ctx.data().database).await?;

    let mut delete_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "DELETE FROM balance WHERE server_id = {id}; DELETE FROM transactions WHERE server_id = {id};",
        id = guild_id.get()
    ));

    let delete_query = delete_query_builder.build();
    println!("{}", delete_query.sql());
    delete_query.execute(&ctx.data().database).await?;

    let members = guild_id.members(&ctx, Some(100), None).await?;

    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO balance (user_id, server_id, baguettes) VALUES\n");

    let mut i = 1;
    let users = members
        .iter()
        .filter(|&x| !x.user.bot)
        .collect::<Vec<&Member>>();
    let len = users.len();
    for member in users {
        query_builder.push(format!(
            "({}, {}, 100)",
            member.user.id.get(),
            guild_id.get()
        ));
        if i < len {
            query_builder.push(",\n");
        }

        i += 1;
    }
    query_builder.push(';');

    let query = query_builder.build();
    println!("{}", query.sql());
    query.execute(&ctx.data().database).await?;

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

    // update_channel(&ctx).await?;
    Ok(())
}
