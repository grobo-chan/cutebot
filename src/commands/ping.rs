/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::{Context, Error};
// use poise::serenity_prelude as serenity;

/// Does Ping-Pong
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}
