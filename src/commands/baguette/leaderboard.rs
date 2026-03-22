use crate::{Context, Data, Error};

use poise::serenity_prelude as serenity;
use serenity::all::{CacheHttp, Mentionable, UserId};
use sqlx::{QueryBuilder, Row, Sqlite};

pub async fn get_pages(guild_id: serenity::GuildId, data: &Data) -> Result<Vec<String>, Error> {
    let mut rows_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT user_id, baguettes FROM balance WHERE server_id = {} ORDER BY baguettes desc;",
        guild_id.get()
    ));

    let rows = rows_query_builder.build().fetch_all(&data.database).await?;
    let l = rows.len();

    let mut pages: Vec<String> = vec![];
    let mut page = String::new();
    for (i, row) in rows.iter().enumerate() {
        let id: u64 = row.try_get("user_id")?;
        let baguettes: u64 = row.try_get("baguettes")?;
        page.push_str(format!("{}. {} {}\n", i + 1, UserId::new(id).mention(), baguettes).as_str());

        if ((i + 1) % 10 == 0) | (i == l - 1) {
            pages.push(page.clone());
            page = String::new();
        }
    }

    Ok(pages)
}

pub async fn get_embed(
    pages: &Vec<String>,
    embed_author: Option<serenity::CreateEmbedAuthor>,
    ctx_id: u64,
) -> Result<(serenity::CreateEmbed, serenity::CreateActionRow), Error> {
    let prev_button_id = format!("{}prev", ctx_id.clone());
    let next_button_id = format!("{}next", ctx_id.clone());

    let author = embed_author.unwrap_or(serenity::CreateEmbedAuthor::new(""));

    // Send the embed with the first page as content
    let components = serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(&prev_button_id).emoji('◀'),
        serenity::CreateButton::new(&next_button_id).emoji('▶'),
    ]);

    let embed = serenity::CreateEmbed::default()
        .description(&pages[0])
        .title("Leaderboard")
        .author(author.clone());

    return Ok((embed, components));
}

pub async fn paginate(
    ctx: &serenity::Context,
    pages: &Vec<String>,
    embed_author: Option<serenity::CreateEmbedAuthor>,
    ctx_id: u64,
) -> Result<(), Error> {
    let prev_button_id = format!("{}prev", ctx_id.clone());
    let next_button_id = format!("{}next", ctx_id.clone());

    let author = embed_author.unwrap_or(serenity::CreateEmbedAuthor::new(""));

    // Loop through incoming interactions with the navigation buttons
    let mut current_page = 0;
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(tokio::time::Duration::from_secs(3600 * 24))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            // This is an unrelated button interaction
            continue;
        }

        // Update the message with the new page contents
        press
            .create_response(
                ctx,
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new().embed(
                        serenity::CreateEmbed::new()
                            .description(&pages[current_page])
                            .title("Leaderboard")
                            .author(author.clone()),
                    ),
                ),
            )
            .await?;
    }

    Ok(())
}

/// The baguette leaderboard
#[poise::command(slash_command, prefix_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    if let Some(guild_id) = ctx.guild_id() {
        let pages = get_pages(guild_id, &ctx.data()).await?;
        let (embed, components) = get_embed(&pages, Some(author.clone()), ctx.id()).await?;

        let reply = poise::CreateReply::default()
            .embed(embed)
            .components(vec![components]);
        ctx.send(reply).await?;

        paginate(
            &ctx.serenity_context(),
            &pages,
            Some(author.clone()),
            ctx.id(),
        )
        .await?;
    }

    Ok(())
}
