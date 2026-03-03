use crate::utils::processor::processor;
use crate::utils::tokenizer::tokenizer;
use crate::{Context, Error};

/// Literally just a calculator with a dice function
#[poise::command(slash_command, prefix_command)]
pub async fn calc(ctx: Context<'_>, expr: String) -> Result<(), Error> {
    ctx.say(format!(
        "The result is: {}",
        processor(tokenizer(expr.as_str()))
    ))
    .await?;
    Ok(())
}
