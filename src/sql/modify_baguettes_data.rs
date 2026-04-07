use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use sqlx::{QueryBuilder, Sqlite};

pub async fn add_baguettes(
    amount: u16,
    user_id: serenity::UserId,
    server_id: serenity::GuildId,
    data: &Data,
) -> Result<(), Error> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "UPDATE balance SET baguettes = baguettes + {amount} WHERE user_id = {user_id} AND server_id = {server_id};",
        amount = amount,
        user_id = user_id.get(),
        server_id = server_id.get()
    ));

    query.build().execute(&data.database).await?;
    Ok(())
}

pub async fn remove_baguettes(
    amount: u16,
    user_id: serenity::UserId,
    server_id: serenity::GuildId,
    data: &Data,
) -> Result<(), Error> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "UPDATE balance SET baguettes = baguettes - {amount} WHERE user_id = {user_id} AND server_id = {server_id};",
        amount = amount,
        user_id = user_id.get(),
        server_id = server_id.get()
    ));

    query.build().execute(&data.database).await?;
    Ok(())
}

pub async fn perform_transaction(
    amount: u16,
    sender_id: serenity::UserId,
    receiver_id: serenity::UserId,
    server_id: serenity::GuildId,
    data: &Data,
) -> Result<(), Error> {
    add_baguettes(amount, receiver_id, server_id, data).await?;
    remove_baguettes(amount, sender_id, server_id, data).await?;

    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "INSERT INTO transactions (transaction_id, server_id, sender_id, receiver_id, amount) values ({transaction_id},{server_id},{sender_id},{receiver_id},{amount});",
        amount = amount,
        sender_id = sender_id.get(),
        receiver_id = receiver_id.get(),
        server_id = server_id.get(),
        transaction_id = rand::random::<u32>()
    ));

    query.build().execute(&data.database).await?;
    Ok(())
}
