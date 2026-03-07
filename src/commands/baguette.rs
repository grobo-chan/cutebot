use crate::{Context, Error};

use poise::serenity_prelude as serenity;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Execute, Sqlite};

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

/// Sets everyone at 100 baguettes
#[poise::command(slash_command, prefix_command, required_permissions = "ADMINISTRATOR")]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    sqlx::query!("DELETE FROM balance;")
        .execute(&ctx.data().database)
        .await
        .unwrap();

    sqlx::query!("DELETE FROM transactions;")
        .execute(&ctx.data().database)
        .await
        .unwrap();

    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let members = guild_id.members(&ctx, Some(100), None).await?;

    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO balance (user_id, money) VALUES\n");

    let mut i = 1;
    let len = members.len();
    for member in members {
        if !member.user.bot {
            query_builder.push(format!("({}, 100)", member.user.id.get()));
            if i < len {
                query_builder.push(",\n");
            }
        }

        i += 1;
    }
    query_builder.push(';');

    let query = query_builder.build();
    let query_str: String = query.sql().into();

    println!("{}", query_str);
    query.execute(&ctx.data().database).await?;

    ctx.say(format!("{}", query_str)).await?;

    Ok(())
}
