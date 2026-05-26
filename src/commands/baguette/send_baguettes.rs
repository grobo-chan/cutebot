/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/
use crate::{
    Context, Error,
    sql::{edit_baguettes_data::perform_transaction, get_baguettes_data::get_user_baguettes_data},
};
use poise::serenity_prelude as serenity;
use serenity::all::Mentionable;

/// Send baguettes to someone
#[poise::command(slash_command, prefix_command, rename = "send")]
pub async fn send_baguettes(
    ctx: Context<'_>,
    receiver: serenity::User,
    amount: u16,
) -> Result<(), Error> {
    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    let embed = {
        if let Some(guild_id) = ctx.guild_id() {
            let sender_balance =
                get_user_baguettes_data(guild_id, ctx.author().id, ctx.data()).await?;

            if receiver.bot {
                serenity::CreateEmbed::new()
                    .author(embed_author)
                    .colour(serenity::Colour::RED)
                    .description(format!(
                        "Error sending baguettes, you can't send money to a bot ({})",
                        receiver.id.mention()
                    ))
            } else if receiver.id.get() == ctx.author().id.get() {
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
                let sender_id = ctx.author().id;
                let receiver_id = receiver.id;
                perform_transaction(amount, sender_id, receiver_id, guild_id, &ctx.data()).await?;

                serenity::CreateEmbed::new()
                    .author(embed_author)
                    .colour(serenity::Colour::DARK_GREEN)
                    .description(format!(
                        "{} baguette(s) have been given by {} to {}",
                        amount,
                        sender_id.mention(),
                        receiver_id.mention()
                    ))
            }
        } else {
            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::RED)
                .description("Not in a server.")
        }
    };

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
