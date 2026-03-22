/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

mod convert;
mod location;

use crate::commands::temp::convert::convert;
use crate::commands::temp::location::location;
use crate::{Context, Error};

struct Temp {
    celsius: f32,
    fahrenheit: f32,
    rankine: f32,
    kelvin: f32,
    reaumur: f32,
}

/// The Parent Temperature Command
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("convert", "location"),
    subcommand_required
)]
pub async fn temp(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
