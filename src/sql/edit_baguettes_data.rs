/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use sqlx::{QueryBuilder, Sqlite};

pub async fn add_daily_baguettes(data: &Data) -> Result<(), Error> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(
        "UPDATE balance SET baguettes = balance.baguettes + servers.daily_baguettes FROM servers WHERE servers.server_id = balance.server_id;",
    );

    query.build().execute(&data.database).await?;

    Ok(())
}

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

pub async fn set_baguettes(
    amount: u16,
    user_id: serenity::UserId,
    server_id: serenity::GuildId,
    data: &Data,
) -> Result<(), Error> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "UPDATE balance SET baguettes = {amount} WHERE user_id = {user_id} AND server_id = {server_id};",
        amount = amount,
        user_id = user_id.get(),
        server_id = server_id.get()
    ));

    query.build().execute(&data.database).await?;
    Ok(())
}

pub async fn add_baguettes_to_all(
    amount: u16,
    server_id: serenity::GuildId,
    data: &Data,
) -> Result<(), Error> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "UPDATE balance SET baguettes = baguettes + {amount} WHERE server_id = {server_id};",
        amount = amount,
        server_id = server_id.get()
    ));

    query.build().execute(&data.database).await?;
    Ok(())
}

pub async fn remove_baguettes_from_all(
    amount: u16,
    server_id: serenity::GuildId,
    data: &Data,
) -> Result<(), Error> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "UPDATE balance SET baguettes = baguettes - {amount} WHERE server_id = {server_id};",
        amount = amount,
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
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "INSERT INTO transactions (transaction_id, server_id, sender_id, receiver_id, amount) values ({transaction_id},{server_id},{sender_id},{receiver_id},{amount});",
        amount = amount,
        sender_id = sender_id.get(),
        receiver_id = receiver_id.get(),
        server_id = server_id.get(),
        transaction_id = rand::random::<u32>()
    ));

    query.build().execute(&data.database).await?;
    add_baguettes(amount, receiver_id, server_id, data).await?;
    remove_baguettes(amount, sender_id, server_id, data).await?;
    Ok(())
}

pub async fn log_action(
    amount: u16,
    user_id: Option<serenity::UserId>,
    admin_id: serenity::UserId,
    server_id: serenity::GuildId,
    action: String,
    data: &Data,
) -> Result<(), Error> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "INSERT INTO baguette_audit_log (action_id, server_id, admin_id, user_id, action, amount) values ({action_id},{server_id},{admin_id},{user_id},'{action}',{amount});",
        amount = amount,
        admin_id = admin_id.get(),
        user_id = user_id.map_or("NULL".to_string(), |v| v.get().to_string()),
        server_id = server_id.get(),
        action_id = rand::random::<u32>(),
        action = action
    ));

    query.build().execute(&data.database).await?;
    Ok(())
}
