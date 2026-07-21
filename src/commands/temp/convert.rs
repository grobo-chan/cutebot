/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use crate::commands::temp::Temp;
use crate::commands::temp::convert::TempAutocomplete::{
    Celsius, Fahrenheit, Kelvin, Rankine, Reaumur,
};
use crate::utils::conversions;
use crate::utils::random_color::random_color;
use crate::{Context, Error};

use poise::serenity_prelude as serenity;

#[derive(poise::ChoiceParameter)]
pub enum TempAutocomplete {
    #[name = "Celsius"]
    #[name = "C"]
    Celsius,
    #[name = "Fahrenheit"]
    #[name = "F"]
    Fahrenheit,
    #[name = "Rankine"]
    #[name = "R"]
    Rankine,
    #[name = "Kelvin"]
    #[name = "K"]
    Kelvin,
    #[name = "Reaumur"]
    #[name = "Re"]
    Reaumur,
}

/// Converts temperatures to show up in other units
#[poise::command(slash_command, prefix_command)]
pub async fn convert(ctx: Context<'_>, number: f32, unit: TempAutocomplete) -> Result<(), Error> {
    let mut temp = Temp {
        celsius: 0.0,
        fahrenheit: 0.0,
        rankine: 0.0,
        kelvin: 0.0,
        reaumur: 0.0,
    };

    match unit {
        Celsius => {
            temp.celsius = number;
            temp.fahrenheit = conversions::celsius::to_fahrenheit(number);
            temp.rankine = conversions::celsius::to_rankine(number);
            temp.kelvin = conversions::celsius::to_kelvin(number);
            temp.reaumur = conversions::celsius::to_reaumur(number);
        }
        Fahrenheit => {
            temp.celsius = conversions::fahrenheit::to_celsius(number);
            temp.fahrenheit = number;
            temp.rankine = conversions::fahrenheit::to_rankine(number);
            temp.kelvin = conversions::fahrenheit::to_kelvin(number);
            temp.reaumur = conversions::fahrenheit::to_reaumur(number);
        }
        Rankine => {
            temp.celsius = conversions::rankine::to_celsius(number);
            temp.fahrenheit = conversions::rankine::to_fahrenheit(number);
            temp.rankine = number;
            temp.kelvin = conversions::rankine::to_kelvin(number);
            temp.reaumur = conversions::rankine::to_reaumur(number);
        }
        Kelvin => {
            temp.celsius = conversions::kelvin::to_celsius(number);
            temp.fahrenheit = conversions::kelvin::to_fahrenheit(number);
            temp.rankine = conversions::kelvin::to_rankine(number);
            temp.kelvin = number;
            temp.reaumur = conversions::kelvin::to_reaumur(number);
        }
        Reaumur => {
            temp.celsius = conversions::reaumur::to_celsius(number);
            temp.fahrenheit = conversions::reaumur::to_fahrenheit(number);
            temp.rankine = conversions::reaumur::to_rankine(number);
            temp.kelvin = conversions::reaumur::to_kelvin(number);
            temp.reaumur = number;
        }
    };

    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    let note = {
        if temp.kelvin < 0.0 {
            "Note: This temperature is below 0 K. Zero Kelvin is the absolute zero, the hypothetical temperature at which the atoms themself stop moving. From the third law of thermodynamics it is impossible to reach it with finite steps."
        } else if temp.kelvin > 1.4e32 {
            "Note: This temperature is above 1.4 × 10³²K. The Planck Temperature is the theoretical maximum temperature possible given our current understanding of physics. Over that temperature the thermal radiation wavelength would be smaller than the Planck Length, the smallest allowed length in our current model of physics"
        } else {
            ""
        }
    };

    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .colour(random_color().await?)
        .title("Temperature")
        .description(format!(
            "The temperature is:\n- {}°C\n- {}°F\n- {} R\n- {} K\n- {} r\n{}",
            temp.celsius, temp.fahrenheit, temp.rankine, temp.kelvin, temp.reaumur, note
        ));
    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
