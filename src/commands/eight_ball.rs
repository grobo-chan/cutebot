/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::utils::random_color::random_color;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

use rand::prelude::*;
use serenity::all::CreateEmbedAuthor;
use serenity::builder::CreateEmbed;

/// Literally just 8ball
#[poise::command(slash_command, prefix_command)]
pub async fn eight_ball(ctx: Context<'_>, question: String) -> Result<(), Error> {
    let embed_author =
        CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name())).icon_url(
            ctx.author()
                .avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url()),
        );

    let answers = vec![
        "It is certain.",
        "It is decidedly so.",
        "Without a doubt.",
        "Yes definitely.",
        "You may rely on it.",
        "As I see it, yes.",
        "Most likely.",
        "Outlook good.",
        "Yes.",
        "Signs point to yes.",
        "Reply hazy, try again.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don't count on it.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Very doubtful.",
    ];

    let answer = {
        match answers.choose(&mut rand::rng()) {
            Some(a) => a,
            None => "Somehow this broke lmao",
        }
    };

    let embed = CreateEmbed::new()
        .author(embed_author)
        .colour(random_color().await?)
        .description(format!(
            "**Question:** {}\n **8-Ball Says:** {}",
            question, answer
        ));

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
