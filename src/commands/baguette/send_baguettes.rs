/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/
use crate::{Context, Error};
use ::serenity::all::Mentionable;
use poise::serenity_prelude as serenity;
use sqlx::{QueryBuilder, Row, Sqlite};

/// Send baguettes to someone
#[poise::command(slash_command, prefix_command, rename = "send")]
pub async fn send_baguettes(
    ctx: Context<'_>,
    send_to: serenity::User,
    amount: u16,
) -> Result<(), Error> {
    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    let mut sender_balance_query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT baguettes FROM balance WHERE server_id = {} AND user_id = {};",
        ctx.guild_id()
            .get_or_insert(serenity::GuildId::new(1))
            .get(),
        ctx.author().id.get()
    ));

    let sender_balance: u16 = sender_balance_query
        .build()
        .fetch_one(&ctx.data().database)
        .await?
        .try_get("baguettes")?;

    let embed = {
        if send_to.bot {
            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::RED)
                .description(format!(
                    "Error sending baguettes, you can't send money to a bot ({})",
                    send_to.id.mention()
                ))
        } else if send_to.id.get() == ctx.author().id.get() {
            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::RED)
                .description("Error sending baguettes, you can't send money to yourself!")
        } else if amount > sender_balance {
            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::RED)
                .description(format!(
                    "Error sending baguettes, amount {} is greater than your balance {}.",
                    amount, sender_balance
                ))
        } else {
            let mut transaction_query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
                "UPDATE balance SET baguettes = baguettes - {amount} WHERE user_id = {sender_id} AND server_id = {server_id}; UPDATE balance SET baguettes = baguettes + {amount} WHERE user_id = {receiver_id} AND server_id = {server_id}; INSERT INTO transactions (transaction_id, server_id, sender_id, receiver_id, amount) values ({transaction_id},{server_id},{sender_id},{receiver_id},{amount});",
                amount = amount,
                sender_id = ctx.author().id.get(),
                receiver_id = send_to.id.get(),
                server_id = ctx
                    .guild_id()
                    .get_or_insert(serenity::GuildId::new(1))
                    .get(),
                transaction_id = rand::random::<u32>()
            ));

            transaction_query
                .build()
                .execute(&ctx.data().database)
                .await?;

            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::DARK_GREEN)
                .description(format!(
                    "{} baguette(s) have been given by {} to {}",
                    amount,
                    ctx.author().id.mention(),
                    send_to.id.mention()
                ))
        }
    };

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
