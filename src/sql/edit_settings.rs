/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use poise::serenity_prelude as serenity;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Execute, Sqlite};

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
