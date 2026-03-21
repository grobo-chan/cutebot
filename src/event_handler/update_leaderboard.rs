use crate::{
    Data, Error,
    commands::baguette::leaderboard::{get_pages, paginate},
};
use futures::StreamExt;
use poise::serenity_prelude as serenity;
use serenity::CacheHttp;
use sqlx::{QueryBuilder, Row, Sqlite};

pub async fn update_channel(
    guild_id: serenity::GuildId,
    ctx: &serenity::Context,
    data: &Data,
) -> Result<(), Error> {
    let mut id_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT leaderboard_channel FROM servers WHERE server_id = {};",
        guild_id
    ));

    // let channel_id: u64 = id_query_builder
    //     .build()
    //     .fetch_one(&data.database)
    //     .await?
    //     .try_get("leaderboard_channel")?;

    let channel_id = 1480154675608027226;

    if channel_id != 0 {
        let leaderboard_channel = serenity::ChannelId::new(channel_id);
        let http = ctx.http();
        let mut messages = leaderboard_channel.messages_iter(&http).boxed();

        while let Some(msg) = messages.next().await {
            match msg {
                Ok(m) => m.delete(&ctx.http()).await?,
                Err(e) => eprintln!("Error deleting message: {}", e),
            }
        }

        let pages = get_pages(guild_id, data).await?;

        paginate(&ctx, None, pages, None, Some(leaderboard_channel)).await?;
    }

    Ok(())
}
