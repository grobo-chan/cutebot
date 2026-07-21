/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

mod edit;
mod get;
mod get_all;

use crate::commands::settings::edit::edit;
use crate::commands::settings::get::get;
use crate::commands::settings::get_all::get_all;
use crate::{Context, Error};

/// The Parent Settings Command
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    subcommands("edit", "get", "get_all"),
    subcommand_required
)]
pub async fn settings(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

// async fn autocomplete(_ctx: Context<'_>, _partial: &str) -> Vec<serenity::AutocompleteChoice> {
//     vec![
//         serenity::AutocompleteChoice::new("Leaderboard Channel", "leaderboard_channel"),
//         serenity::AutocompleteChoice::new("Landmine Channel", "landmine_channel"),
//         serenity::AutocompleteChoice::new("Landmine Immunity Role", "landmine_immunity_role"),
//         serenity::AutocompleteChoice::new("Gambling Enabled", "gambling_enabled"),
//         serenity::AutocompleteChoice::new("Daily Baguettes", "daily_baguettes"),
//         serenity::AutocompleteChoice::new("America mode Enabled", "america_mode"),
//     ]
//     .into_iter()
//     // .filter(move |name| futures::future::ready(name.starts_with(partial)))
//     // .map(|name| name.to_string())
// }

#[derive(poise::ChoiceParameter)]
pub enum SettingsAutocomplete {
    #[name = "Leaderboard Channel"]
    LeaderboardChannel,
    #[name = "Landmine Channel"]
    LandmineChannel,
    #[name = "Landmine Immunity Role"]
    LandmineImmunityRole,
    #[name = "Gambling Enabled"]
    GamblingEnabled,
    #[name = "Daily Baguettes"]
    DailyBaguettes,
    #[name = "America mode Enabled"]
    AmericaModeEnabled,
}
