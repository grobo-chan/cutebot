/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use poise::serenity_prelude as serenity;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Column, Execute, Row, Sqlite};

use crate::{Data, Error};

pub async fn edit_setting<T: std::fmt::Display, U: std::fmt::Display>(
    guild_id: serenity::GuildId,
    setting: T,
    new_value: U,
    data: &Data,
) -> Result<(), Error> {
    let mut edit_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "UPDATE servers SET {} = {} WHERE server_id = {};",
        setting,
        new_value,
        guild_id.get()
    ));

    let edit_query = edit_query_builder.build();
    println!("{}", edit_query.sql());
    edit_query.execute(&data.database).await?;

    Ok(())
}

pub async fn get_setting(
    guild_id: serenity::GuildId,
    setting: &str,
    data: &Data,
) -> Result<String, Error> {
    let query_str = format!(
        "SELECT CAST({0} AS TEXT) AS {0} FROM servers WHERE server_id = $1;",
        setting
    );
    let row = sqlx::query(&query_str)
        .bind(&guild_id.get().to_string())
        .fetch_one(&data.database)
        .await?;

    for c in row.columns() {
        println!("{}", c.name());
    }

    let result = row.try_get::<String, &str>(setting).unwrap_or_default();
    Ok(result)
}
