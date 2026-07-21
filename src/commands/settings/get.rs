/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::commands::settings::SettingsAutocomplete::{
    self, AmericaModeEnabled, DailyBaguettes, GamblingEnabled, LandmineChannel,
    LandmineImmunityRole, LeaderboardChannel,
};
use crate::sql::settings::get_setting;
use crate::{Context, Error};

use poise::serenity_prelude as serenity;
use serenity::all::Mentionable;

/// Gets value of a setting
#[poise::command(slash_command, prefix_command)]
pub async fn get(ctx: Context<'_>, option: SettingsAutocomplete) -> Result<(), Error> {
    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );
    let server_id = ctx.guild_id().expect("Not in a server!");

    let embed = match option {
        LeaderboardChannel | LandmineChannel => {
            let db_name = match option {
                LeaderboardChannel => "leaderboard_channel",
                LandmineChannel => "landmine_channel",
                _ => "",
            };

            let value = get_setting(server_id, db_name, &ctx.data()).await?;

            let option_name = match option {
                LeaderboardChannel => "Leaderboard Channel",
                LandmineChannel => "Landmine Channel",
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
        LandmineImmunityRole => {
            let value = get_setting(server_id, "landmine_immunity_role", &ctx.data()).await?;
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
        GamblingEnabled | AmericaModeEnabled => {
            let db_name = match option {
                GamblingEnabled => "gambling_enabled",
                AmericaModeEnabled => "america_mode",
                _ => "",
            };

            let value = get_setting(server_id, db_name, &ctx.data()).await?;

            let option_name = match option {
                GamblingEnabled => "Gambling",
                AmericaModeEnabled => "America mode",
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
        DailyBaguettes => {
            let value = get_setting(server_id, "daily_baguettes", &ctx.data()).await?;
            let daily_baguettes: i32 = value.parse().unwrap();

            serenity::CreateEmbed::new()
                .author(embed_author)
                .colour(serenity::Colour::DARK_GREEN)
                .description(format!(
                    "The current number of daily baguettes is {}",
                    daily_baguettes
                ))
        }
    };

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
