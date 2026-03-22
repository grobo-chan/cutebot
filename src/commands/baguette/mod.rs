/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

pub mod leaderboard;
mod reset;

use crate::commands::baguette::leaderboard::leaderboard;
use crate::commands::baguette::reset::reset;
use crate::{Context, Error};

/// The Parent Baguette Command
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("reset", "leaderboard"),
    subcommand_required
)]
pub async fn baguette(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
