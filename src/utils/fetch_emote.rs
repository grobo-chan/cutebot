/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::Error;
use poise::serenity_prelude as serenity;

pub async fn fetch_emote(http: &serenity::Http, name: String) -> Result<String, Error> {
    let app_emojis = http.get_application_emojis().await?;
    let emoji = app_emojis
        .iter()
        .find(|e| e.name == name)
        .map(|e| {
            if e.animated {
                format!("<a:{}:{}>", e.name, e.id)
            } else {
                format!("<:{}:{}>", e.name, e.id)
            }
        })
        .unwrap_or_else(|| format!(":{}:", name));

    Ok(emoji)
}
