/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::sql::settings::get_all_settings;
use crate::{Context, Error};

use ::serenity::all::prelude::Mentionable;
use poise::serenity_prelude as serenity;

/// Gets value of a setting
#[poise::command(slash_command, prefix_command)]
pub async fn get_all(ctx: Context<'_>) -> Result<(), Error> {
    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );
    let server_id = ctx.guild_id().expect("Not in a server!");
    let values = get_all_settings(server_id, &ctx.data()).await?;

    let description = {
        let mut final_str = String::from("");

        for [i, j] in values {
            match i.as_str() {
                "leaderboard_channel" | "landmine_channel" => {
                    let option_name = match i.as_str() {
                        "leaderboard_channel" => "Leaderboard Channel",
                        "landmine_channel" => "Landmine Channel",
                        _ => "",
                    };

                    let id: u64 = j.parse().unwrap_or_default();
                    if id != 0 {
                        let channel = serenity::ChannelId::new(id);
                        final_str.push_str(&format!(
                            "**{}:** {}\n",
                            option_name,
                            channel.mention()
                        ));
                    } else {
                        final_str.push_str(&format!("**{}:** Not set\n", option_name));
                    }
                }
                "landmine_immunity_role" => {
                    let id: u64 = j.parse().unwrap_or_default();
                    if id != 0 {
                        let role = serenity::RoleId::new(id);
                        final_str
                            .push_str(&format!("**Landmine Immunity Role:** {}\n", role.mention()));
                    } else {
                        final_str.push_str("**Landmine Immunity Role:** Not set\n");
                    }
                }
                "gambling_enabled" | "america_mode" => {
                    let option_name = match i.as_str() {
                        "gambling_enabled" => "Gambling",
                        "america_mode" => "America mode",
                        _ => "",
                    };

                    let is_enabled = if j == "1".to_string() {
                        "enabled"
                    } else {
                        "disabled"
                    };

                    final_str.push_str(&format!("**{}:** {}\n", option_name, is_enabled));
                }
                "daily_baguettes" => {
                    final_str.push_str(&format!("**Daily Baguettes:** {}\n", j));
                }
                _ => {}
            }
        }

        final_str
    };

    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .title("Settings")
        .description(description);

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
