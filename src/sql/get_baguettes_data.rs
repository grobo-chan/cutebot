/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::{Data, Error};
use ::serenity::all::Mentionable;
use poise::serenity_prelude as serenity;
use sqlx::{QueryBuilder, Row, Sqlite};

pub async fn get_all_baguettes_data(
    guild_id: serenity::GuildId,
    data: &Data,
) -> Result<Vec<(String, u64)>, Error> {
    let mut rows_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT user_id, baguettes FROM balance WHERE server_id = {} ORDER BY baguettes desc;",
        guild_id.get()
    ));

    let rows = rows_query_builder.build().fetch_all(&data.database).await?;
    let mut info: Vec<(String, u64)> = vec![];
    for i in rows.iter() {
        let id: u64 = i.try_get("user_id")?;
        let baguettes: u64 = i.try_get("baguettes")?;
        info.push((
            format!("{}", serenity::UserId::new(id).mention()),
            baguettes,
        ));
    }

    Ok(info)
}

pub async fn get_user_baguettes_data(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    data: &Data,
) -> Result<u16, Error> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT baguettes FROM balance WHERE server_id = {} AND user_id = {};",
        guild_id.get(),
        user_id.get()
    ));

    let balance: u16 = query
        .build()
        .fetch_one(&data.database)
        .await?
        .try_get("baguettes")?;

    Ok(balance)
}
