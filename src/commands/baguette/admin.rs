use crate::{
    Context, Error,
    sql::modify_baguettes_data::{add_baguettes, log_action, remove_baguettes, set_baguettes},
};
use ::serenity::all::Mentionable;
use poise::serenity_prelude as serenity;

/// The Parent Baguette Command
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    subcommands("add_baguette", "remove_baguette", "set_baguette"),
    subcommand_required
)]
pub async fn admin(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Add Baguettes
#[poise::command(slash_command, prefix_command)]
pub async fn add_baguette(
    ctx: Context<'_>,
    user: serenity::User,
    amount: u16,
) -> Result<(), Error> {
    let admin_id = ctx.author().id;
    let user_id = user.id;
    let server_id = {
        match ctx.guild_id() {
            Some(id) => id,
            _ => panic!("Not in a server!"),
        }
    };
    let action = String::from("add_baguettes");

    log_action(amount, user_id, admin_id, server_id, action, &ctx.data()).await?;
    add_baguettes(amount, user_id, server_id, &ctx.data()).await?;

    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .colour(serenity::Colour::DARK_GREEN)
        .description(format!(
            "{} baguette(s) were given to {}",
            amount,
            user_id.mention()
        ));

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

/// Add Baguettes
#[poise::command(slash_command, prefix_command)]
pub async fn remove_baguette(
    ctx: Context<'_>,
    user: serenity::User,
    amount: u16,
) -> Result<(), Error> {
    let admin_id = ctx.author().id;
    let user_id = user.id;
    let server_id = {
        match ctx.guild_id() {
            Some(id) => id,
            _ => panic!("Not in a server!"),
        }
    };
    let action = String::from("remove_baguettes");

    log_action(amount, user_id, admin_id, server_id, action, &ctx.data()).await?;
    remove_baguettes(amount, user_id, server_id, &ctx.data()).await?;

    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .colour(serenity::Colour::DARK_GREEN)
        .description(format!(
            "{} baguette(s) were removed from {}",
            amount,
            user_id.mention()
        ));

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

/// Add Baguettes
#[poise::command(slash_command, prefix_command)]
pub async fn set_baguette(
    ctx: Context<'_>,
    user: serenity::User,
    amount: u16,
) -> Result<(), Error> {
    let admin_id = ctx.author().id;
    let user_id = user.id;
    let server_id = {
        match ctx.guild_id() {
            Some(id) => id,
            _ => panic!("Not in a server!"),
        }
    };
    let action = String::from("set_baguettes");

    log_action(amount, user_id, admin_id, server_id, action, &ctx.data()).await?;
    set_baguettes(amount, user_id, server_id, &ctx.data()).await?;

    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .colour(serenity::Colour::DARK_GREEN)
        .description(format!(
            "{} now has {} baguette(s)",
            user_id.mention(),
            amount
        ));

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
