use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Buns a user
#[poise::command(slash_command, prefix_command)]
pub async fn ben(
    ctx: Context<'_>,
    #[description = "Selected User"] user: serenity::User,
) -> Result<(), Error> {
    let response = format!("{} has been bunned!", user.name);
    ctx.say(response).await?;
    Ok(())
}
