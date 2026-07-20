/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use poise::serenity_prelude as serenity;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Column, Row, Sqlite};

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
    edit_query.execute(&data.database).await?;

    Ok(())
}

pub async fn get_setting(
    guild_id: serenity::GuildId,
    setting: &str,
    data: &Data,
) -> Result<String, Error> {
    let mut get_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT CAST({0} AS TEXT) AS {0} FROM servers WHERE server_id = {1};",
        setting,
        guild_id.get()
    ));

    let row = get_query_builder.build().fetch_one(&data.database).await?;

    let result = row.try_get::<String, &str>(setting).unwrap_or_default();
    Ok(result)
}

pub async fn get_all_settings(
    guild_id: serenity::GuildId,
    data: &Data,
) -> Result<Vec<[String; 2]>, Error> {
    let mut get_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT * FROM servers WHERE server_id = {0};",
        guild_id.get()
    ));

    let row = get_query_builder.build().fetch_one(&data.database).await?;
    let result: Vec<[String; 2]> = row
        .columns()
        .iter()
        .map(|x| {
            [
                x.name().to_string(),
                row.try_get_unchecked::<String, &str>(x.name())
                    .unwrap_or_default(),
            ]
        })
        .collect();

    Ok(result)
}
