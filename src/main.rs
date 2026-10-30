mod commands;
mod db;
mod game;
mod models;

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

pub struct Data {
    pub db: Arc<Mutex<rusqlite::Connection>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            error!("Fatal error during bot initialization: {:?}", error);
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!("Error in command '{}': {:?}", ctx.command().name, error);
            let _ = ctx.say(format!("⚠️ An internal error occurred: {}", error)).await;
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Unhandled error callback failure: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env if present
    dotenvy::dotenv().ok();

    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,avengers_bot=debug".into()),
        )
        .init();

    info!("Initializing Avengers S.H.I.E.L.D. Database...");
    let conn = rusqlite::Connection::open("avengers.db")?;
    db::init_db(&conn)?;
    let db = Arc::new(Mutex::new(conn));
    info!("Database initialized successfully.");

    let token = std::env::var("DISCORD_TOKEN").expect(
        "Expected DISCORD_TOKEN in environment or .env file. Please provide your Discord Bot Token.",
    );

    let intents = serenity::all::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::start(),
                commands::profile(),
                commands::roster(),
                commands::switch(),
                commands::battle(),
                commands::raid(),
                commands::mission(),
                commands::shop(),
                commands::buy(),
                commands::daily(),
                commands::leaderboard(),
                commands::help(),
            ],
            on_error: |err| Box::pin(on_error(err)),
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("Logged in as {} (ID: {})", ready.user.name, ready.user.id);
                info!("Registering slash commands globally...");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("All Avengers slash commands registered successfully!");

                Ok(Data { db })
            })
        })
        .build();

    let client = serenity::all::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    info!("Starting S.H.I.E.L.D. Avengers Bot daemon...");
    client?.start().await?;

    Ok(())
}
