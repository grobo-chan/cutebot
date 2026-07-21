/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::commands::settings::autocomplete;
use crate::sql::settings::get_setting;
use crate::{Context, Error};

use poise::serenity_prelude as serenity;
use serenity::all::Mentionable;

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
        "gambling_enabled" | "america_mode" => {
            let option_name = match option.to_lowercase().as_str() {
                "gambling_enabled" => "Gambling",
                "america_mode" => "America mode",
                _ => "",
            };

            let is_enabled = if value == "1".to_string() {
                "enabled"
            } else {
                "disabled"
            };

            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::DARK_GREEN)
                .description(format!("{} is {}", option_name, is_enabled))
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
