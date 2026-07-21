/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::Error;
use poise::serenity_prelude as serenity;
use rand::prelude::*;

pub async fn random_color() -> Result<serenity::Color, Error> {
    let r: u8 = rand::rng().random_range(0..=255);
    let g: u8 = rand::rng().random_range(0..=255);
    let b: u8 = rand::rng().random_range(0..=255);

    Ok(serenity::Colour::from_rgb(r, g, b))
}
