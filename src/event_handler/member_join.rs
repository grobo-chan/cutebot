use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use sqlx::{QueryBuilder, Sqlite};

pub async fn member_join(new_member: &serenity::all::Member, data: &Data) -> Result<(), Error> {
    let user_id = new_member.user.id.get();
    let guild_id = new_member.guild_id;
    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT OR IGNORE INTO balance (user_id, server_id, baguettes) VALUES");
    query_builder.push(format!(" ({}, {}, 100);", user_id, guild_id));

    let query = query_builder.build();
    query.execute(&data.database).await?;

    Ok(())
}
