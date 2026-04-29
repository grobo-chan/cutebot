/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use futures::StreamExt;
use poise::serenity_prelude as serenity;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Execute, Sqlite};

use crate::{Data, Error};

pub async fn reset_server(
    guild_id: &serenity::GuildId,
    data: &Data,
    http: &serenity::all::Http,
) -> Result<(), Error> {
    let mut settings_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "INSERT OR IGNORE INTO servers VALUES ({}, NULL, NULL, NULL, NULL, 0);",
        guild_id.get()
    ));

    let settings_query = settings_query_builder.build();
    println!("{}", settings_query.sql());
    settings_query.execute(&data.database).await?;

    let mut delete_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "DELETE FROM balance WHERE server_id = {id};",
        id = guild_id.get()
    ));
    let delete_query = delete_query_builder.build();
    println!("{}", delete_query.sql());
    delete_query.execute(&data.database).await?;

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("");

    let mut members = guild_id.members_iter(http).boxed();
    while let Some(member) = members.next().await {
        match member {
            Ok(m) => {
                if !m.user.bot {
                    query_builder.push(format!(
                        "INSERT INTO balance (user_id, server_id, baguettes) VALUES ({}, {}, 100);",
                        m.user.id.get(),
                        guild_id.get()
                    ));
                }
            }
            Err(e) => panic!("Error {} while resetting server {}", e, guild_id.get()),
        }
    }

    let query = query_builder.build();
    println!("{}", query.sql());
    query.execute(&data.database).await?;

    Ok(())
}
