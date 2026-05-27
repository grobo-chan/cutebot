/*
Copyright (C) 2026 GroboChan

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

mod commands;
mod event_handler;
mod sql;
mod utils;

use poise::serenity_prelude as serenity;
use sqlx::{Pool, Sqlite};
use std::env;
use std::sync::Arc;
use std::time::Duration;

use crate::event_handler::event_handler;
use crate::sql::edit_baguettes_data::add_daily_baguettes;

pub struct Data {
    database: Pool<Sqlite>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            eprintln!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                eprintln!("Error while handling error: {}", e)
            }
        }
    }
}

async fn background_task(data: Arc<Data>) {
    let mut interval = tokio::time::interval(Duration::from_hours(24));
    // let mut interval = tokio::time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;
        if let Err(e) = add_daily_baguettes(&Data {
            database: data.database.clone(),
        })
        .await
        {
            eprintln!("Error in background task: {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to the DB");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::age::age(),
                commands::ping::ping(),
                commands::ben::ben(),
                commands::calc::calc(),
                commands::temp::temp(),
                commands::baguette::baguette(),
                commands::getemote::getemote(),
                commands::eight_ball::eight_ball(),
                commands::settings::settings(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let arc_data = Arc::new(Data {
                    database: database.clone(),
                });

                tokio::spawn(async move {
                    background_task(arc_data).await;
                });

                Ok(Data { database: database })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
