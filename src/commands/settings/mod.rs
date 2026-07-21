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
