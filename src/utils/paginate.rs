/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::{Error, utils::random_color::random_color};
use futures::StreamExt;
use poise::serenity_prelude as serenity;

pub async fn get_pages<T: std::fmt::Display, U: std::fmt::Display>(
    data: Vec<(T, U)>,
) -> Result<Vec<String>, Error> {
    let l = data.len();

    let mut pages: Vec<String> = vec![];
    let mut page = String::new();
    for (i, row) in data.iter().enumerate() {
        let (x, y) = row;
        page.push_str(format!("{}. {} {}\n", i + 1, x, y).as_str());

        if ((i + 1) % 10 == 0) | (i + 1 == l) {
            pages.push(page.clone());
            page = String::new();
        }
    }

    Ok(pages)
}

pub async fn get_first_page_embed(
    pages: &Vec<String>,
    embed_author: Option<serenity::CreateEmbedAuthor>,
    ctx_id: u64,
) -> Result<(serenity::CreateEmbed, serenity::CreateActionRow), Error> {
    let prev_button_id = format!("{}prev", ctx_id.clone());
    let next_button_id = format!("{}next", ctx_id.clone());

    let author = embed_author.unwrap_or(serenity::CreateEmbedAuthor::new(""));

    let components = serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(&prev_button_id).emoji('◀'),
        serenity::CreateButton::new(&next_button_id).emoji('▶'),
    ]);

    let embed = serenity::CreateEmbed::default()
        .description(&pages[0])
        .title("Leaderboard")
        .color(random_color().await?)
        .author(author.clone());

    return Ok((embed, components));
}

pub async fn paginate_embed_message(
    ctx: &serenity::Context,
    pages: &Vec<String>,
    embed_author: Option<serenity::CreateEmbedAuthor>,
    timeout: bool,
    ctx_id: u64,
) -> Result<(), Error> {
    let prev_button_id = format!("{}prev", ctx_id.clone());
    let next_button_id = format!("{}next", ctx_id.clone());

    let author = embed_author.unwrap_or(serenity::CreateEmbedAuthor::new(""));

    // Loop through incoming interactions with the navigation buttons
    let mut current_page = 0;
    let mut collector = serenity::collector::ComponentInteractionCollector::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()));
    if timeout {
        // Timeout when no navigation button has been pressed for 24 hours
        collector = collector.timeout(tokio::time::Duration::from_secs(3600 * 24))
    }

    let mut stream = collector.stream();
    while let Some(press) = stream.next().await {
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
