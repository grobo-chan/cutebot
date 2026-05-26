/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::sql::settings::{edit_setting, get_setting};
use crate::{Context, Error};

use futures::{Stream, StreamExt};
use poise::serenity_prelude as serenity;
use serenity::all::Mentionable;
use serenity::utils::{parse_channel_mention, parse_role_mention};

/// The Parent Settings Command
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    subcommands("edit", "get"),
    subcommand_required
)]
pub async fn settings(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

async fn autocomplete<'a>(_ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> {
    futures::stream::iter(&[
        "leaderboard_channel",
        "landmine_channel",
        "landmine_immunity_role",
        "gambling_enabled",
        "daily_baguettes",
    ])
    .filter(move |name| futures::future::ready(name.starts_with(partial)))
    .map(|name| name.to_string())
}

/// Edits a setting
#[poise::command(slash_command, prefix_command)]
pub async fn edit(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete"] option: String,
    new_setting: String,
) -> Result<(), Error> {
    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );
    let server_id = ctx.guild_id().expect("Not in a server!");

    let embed = match option.to_lowercase().as_str() {
        "leaderboard_channel" | "landmine_channel" => {
            let channel_id = parse_channel_mention(new_setting.as_str());
            match channel_id {
                Some(c) => {
                    edit_setting(server_id, &option, c.get(), &ctx.data()).await?;

                    let option_name = match option.to_lowercase().as_str() {
                        "leaderboard_channel" => "Leaderboard Channel",
                        "landmine_channel" => "Landmine Channel",
                        _ => "",
                    };

                    serenity::CreateEmbed::new()
                        .author(embed_author)
                        .colour(serenity::Colour::DARK_GREEN)
                        .description(format!("{} is now set to {}", option_name, c.mention()))
                }
                None => serenity::CreateEmbed::new()
                    .author(embed_author)
                    .colour(serenity::Colour::RED)
                    .description("Invalid channel id was provided!"),
            }
        }
        "landmine_immunity_role" => {
            let role_id = parse_role_mention(new_setting.as_str());
            match role_id {
                Some(r) => {
                    edit_setting(server_id, &option, r.get(), &ctx.data()).await?;

                    serenity::CreateEmbed::new()
                        .author(embed_author)
                        .colour(serenity::Colour::DARK_GREEN)
                        .description(format!(
                            "Landmine immunity role is now set to {}",
                            r.mention()
                        ))
                }
                None => serenity::CreateEmbed::new()
                    .author(embed_author)
                    .colour(serenity::Colour::RED)
                    .description("Invalid role id was provided!"),
            }
        }
        "gambling_enabled" => match new_setting.to_lowercase().as_str() {
            "yes" | "y" | "true" => {
                edit_setting(server_id, &option, true, &ctx.data()).await?;

                serenity::CreateEmbed::new()
                    .author(embed_author)
                    .colour(serenity::Colour::DARK_GREEN)
                    .description("Gambling is now enabled!")
            }
            "no" | "n" | "false" => {
                edit_setting(server_id, &option, false, &ctx.data()).await?;

                serenity::CreateEmbed::new()
                    .author(embed_author)
                    .colour(serenity::Colour::DARK_GREEN)
                    .description("Gambling is now disabled!")
            }
            _ => serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::RED)
                .description("Please pick either 'yes' or 'no'."),
        },
        "daily_baguettes" => match new_setting.parse::<i32>().ok() {
            Some(amount) => {
                edit_setting(server_id, &option, amount, &ctx.data()).await?;

                serenity::CreateEmbed::new()
                    .author(embed_author)
                    .colour(serenity::Colour::DARK_GREEN)
                    .description(format!(
                        "The number of daily baguettes is now set to {}",
                        amount
                    ))
            }
            None => serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::RED)
                .description("Please select an integer."),
        },
        _ => serenity::CreateEmbed::new()
            .author(embed_author)
            .colour(serenity::Colour::RED)
            .description("Invalid option was provided!"),
    };

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}

/// Gets value of a setting
#[poise::command(slash_command, prefix_command)]
pub async fn get(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete"] option: String,
) -> Result<(), Error> {
    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );
    let server_id = ctx.guild_id().expect("Not in a server!");
    let value = get_setting(server_id, &option, &ctx.data()).await?;

    let embed = match option.to_lowercase().as_str() {
        "leaderboard_channel" | "landmine_channel" => {
            let option_name = match option.to_lowercase().as_str() {
                "leaderboard_channel" => "Leaderboard Channel",
                "landmine_channel" => "Landmine Channel",
                _ => "",
            };

            let description = if value != "" {
                let channel_id = serenity::ChannelId::new(value.parse().unwrap());
                format!("The {} is {}", option_name, channel_id.mention())
            } else {
                format!("The {} is not set", option_name)
            };

            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::DARK_GREEN)
                .description(description)
        }
        "landmine_immunity_role" => {
            let description = if value != "" {
                let role_id = serenity::RoleId::new(value.parse().unwrap());
                format!(
                    "The current landmine immunity role is {}",
                    role_id.mention()
                )
            } else {
                "The current landmine immunity role is not set".to_string()
            };

            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::DARK_GREEN)
                .description(description)
        }
        "gambling_enabled" => {
            let gambling_enabled = if value == "1".to_string() {
                "enabled"
            } else {
                "disabled"
            };

            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::DARK_GREEN)
                .description(format!("Gambling is {}", gambling_enabled))
        }
        "daily_baguettes" => {
            let daily_baguettes: i32 = value.parse().unwrap();

            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::DARK_GREEN)
                .description(format!(
                    "The current number of daily baguettes is {}",
                    daily_baguettes
                ))
        }
        _ => serenity::CreateEmbed::new()
            .author(embed_author)
            .colour(serenity::Colour::RED)
            .description("Invalid option was provided!"),
    };

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
