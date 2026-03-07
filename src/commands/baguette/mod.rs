pub mod reset;

use crate::commands::baguette::reset::reset;
use crate::{Context, Error};

/// The Parent Baguette Command
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("reset"),
    subcommand_required
)]
pub async fn baguette(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
