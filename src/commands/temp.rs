use crate::utils::conversions;
use crate::{Context, Error};

use futures::{Stream, StreamExt};
use poise::serenity_prelude as serenity;
use serenity::all::CreateEmbedAuthor;
use serenity::builder::CreateEmbed;

async fn autocomplete<'a>(_ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> {
    futures::stream::iter(&["Celsius", "Fahrenheit", "Rankine", "Kelvin", "Reaumur"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

struct Temp {
    celsius: f64,
    fahrenheit: f64,
    rankine: f64,
    kelvin: f64,
    reaumur: f64,
}

/// Converts temperatures to show up in other units
#[poise::command(slash_command, prefix_command)]
pub async fn temp(
    ctx: Context<'_>,
    number: f64,
    #[autocomplete = "autocomplete"] unit: String,
) -> Result<(), Error> {
    let mut temp = Temp {
        celsius: 0.0,
        fahrenheit: 0.0,
        rankine: 0.0,
        kelvin: 0.0,
        reaumur: 0.0,
    };

    let mut invalid = false;
    match unit.to_lowercase().as_str() {
        "celsius" => {
            temp.celsius = number;
            temp.fahrenheit = conversions::celsius::to_fahrenheit(number);
            temp.rankine = conversions::celsius::to_rankine(number);
            temp.kelvin = conversions::celsius::to_kelvin(number);
            temp.reaumur = conversions::celsius::to_reaumur(number);
        }
        "fahrenheit" => {
            temp.celsius = conversions::fahrenheit::to_celsius(number);
            temp.fahrenheit = number;
            temp.rankine = conversions::fahrenheit::to_rankine(number);
            temp.kelvin = conversions::fahrenheit::to_kelvin(number);
            temp.reaumur = conversions::fahrenheit::to_reaumur(number);
        }
        "rankine" => {
            temp.celsius = conversions::rankine::to_celsius(number);
            temp.fahrenheit = conversions::rankine::to_fahrenheit(number);
            temp.rankine = number;
            temp.kelvin = conversions::rankine::to_kelvin(number);
            temp.reaumur = conversions::rankine::to_reaumur(number);
        }
        "kelvin" => {
            temp.celsius = conversions::kelvin::to_celsius(number);
            temp.fahrenheit = conversions::kelvin::to_fahrenheit(number);
            temp.rankine = conversions::kelvin::to_rankine(number);
            temp.kelvin = number;
            temp.reaumur = conversions::kelvin::to_reaumur(number);
        }
        "reaumur" => {
            temp.celsius = conversions::reaumur::to_celsius(number);
            temp.fahrenheit = conversions::reaumur::to_fahrenheit(number);
            temp.rankine = conversions::reaumur::to_rankine(number);
            temp.kelvin = conversions::reaumur::to_kelvin(number);
            temp.reaumur = number;
        }
        _ => {
            invalid = true;
        }
    };

    let embed_author =
        CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name())).icon_url(
            ctx.author()
                .avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url()),
        );

    if !invalid {
        let embed = CreateEmbed::new()
            .author(embed_author)
            .colour(serenity::Colour::DARK_MAGENTA)
            .title("Temperature")
            .description(format!(
                "The temperature is:\n- {}°C\n- {}°F\n- {} R\n- {} K\n- {} r",
                temp.celsius, temp.fahrenheit, temp.rankine, temp.kelvin, temp.reaumur
            ));

        let reply = poise::CreateReply::default().embed(embed);
        ctx.send(reply).await?;
    } else {
        let embed = CreateEmbed::new()
            .author(embed_author)
            .colour(serenity::Colour::RED)
            .title("Error")
            .description("Invalid Unit");

        let reply = poise::CreateReply::default().embed(embed);
        ctx.send(reply).await?;
    }

    Ok(())
}
