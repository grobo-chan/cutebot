use crate::commands::temp::Temp;
use crate::utils::conversions;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct MainStats {
    temp: f32,
    humidity: i32,
}

#[derive(Deserialize)]
struct Wind {
    speed: f32,
}

#[derive(Deserialize)]
struct WeatherResponse {
    name: String,
    main: MainStats,
    wind: Wind,
}

/// Gets temp of a location
#[poise::command(slash_command, prefix_command)]
pub async fn location(ctx: Context<'_>, city: String) -> Result<(), Error> {
    let openweather_key = env::var("OPENWEATHER_KEY").expect("Missing OpenWeather API KEY");
    let params = [("q", city), ("appid", openweather_key)];
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.openweathermap.org/data/2.5/weather")
        .query(&params)
        .send()
        .await?;

    let body = res.json::<WeatherResponse>().await?;

    let temp = Temp {
        celsius: conversions::kelvin::to_celsius(body.main.temp),
        fahrenheit: conversions::kelvin::to_fahrenheit(body.main.temp),
        kelvin: body.main.temp,
        rankine: conversions::kelvin::to_rankine(body.main.temp),
        reaumur: conversions::kelvin::to_reaumur(body.main.temp),
    };

    let embed_author =
        serenity::CreateEmbedAuthor::new(&format!("Requested by: {}", ctx.author().display_name()))
            .icon_url(
                ctx.author()
                    .avatar_url()
                    .unwrap_or_else(|| ctx.author().default_avatar_url()),
            );

    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .colour(serenity::Colour::DARK_MAGENTA)
        .title("Temperature")
        .description(format!(
            "The temperature in {} is:\n- {}°C\n- {}°F\n- {} R\n- {} K\n- {} r\nWith a wind speed of {} m/s and {}% Humidity",
            body.name, temp.celsius, temp.fahrenheit, temp.rankine, temp.kelvin, temp.reaumur, body.wind.speed, body.main.humidity
        ));
    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
