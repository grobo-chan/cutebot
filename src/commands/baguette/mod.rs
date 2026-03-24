/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

pub mod leaderboard;
mod reset;
mod send_baguettes;

use crate::commands::baguette::leaderboard::leaderboard;
use crate::commands::baguette::reset::reset;
use crate::commands::baguette::send_baguettes::send_baguettes;
use crate::{Context, Error};

/// The Parent Baguette Command
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("reset", "leaderboard", "send_baguettes"),
    subcommand_required
)]
pub async fn baguette(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
