/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

mod edit;
mod get;

use crate::commands::settings::edit::edit;
use crate::commands::settings::get::get;
use crate::{Context, Error};
use futures::{Stream, StreamExt};

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
        "america_mode",
    ])
    .filter(move |name| futures::future::ready(name.starts_with(partial)))
    .map(|name| name.to_string())
}
