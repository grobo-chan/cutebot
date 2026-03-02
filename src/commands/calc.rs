use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::RngExt;

enum Token {
    Number(f64),
    Dice { count: u32, sides: u32 },
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn dice_to_num(dice: Token) -> Token {
    if let Token::Dice { count, sides } = dice {
        let mut s: f64 = 0.0;
        let mut rng = rand::rng();

        for _ in 0..count {
            s += rng.random_range(1..=sides) as f64;
        }

        return Token::Number(s);
    } else {
        Token::Number(0.0)
    }
}

/// Literally just a calculator with a dice function
#[poise::command(slash_command, prefix_command)]
pub async fn calc(ctx: Context<'_>, expr: String) -> Result<(), Error> {
    ctx.say(expr).await?;
    Ok(())
}
