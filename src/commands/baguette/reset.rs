use crate::{Context, Error};

use poise::serenity_prelude as serenity;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use sqlx::Sqlite;
use sqlx::query_builder::QueryBuilder;

/// Sets everyone at 100 baguettes
#[poise::command(slash_command, prefix_command, required_permissions = "ADMINISTRATOR")]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    sqlx::query!("DELETE FROM balance;")
        .execute(&ctx.data().database)
        .await
        .unwrap();

    sqlx::query!("DELETE FROM transactions;")
        .execute(&ctx.data().database)
        .await
        .unwrap();

    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let members = guild_id.members(&ctx, Some(100), None).await?;

    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO balance (user_id, money) VALUES\n");

    let mut i = 1;
    let len = members.len();
    for member in members {
        if !member.user.bot {
            query_builder.push(format!("({}, 100)", member.user.id.get()));
            if i < len {
                query_builder.push(",\n");
            }
        }

        i += 1;
    }
    query_builder.push(';');

    let query = query_builder.build();
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

    Ok(())
}
