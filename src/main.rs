mod commands;
mod framework;

use std::process::exit;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use serenity::model::id::GuildId;
use thiserror::Error;

#[derive(Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub guild_id: GuildId,
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Figment(#[from] figment::Error),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),
}

async fn run() -> Result<(), Error> {
    let config: Config = Figment::new()
        .merge(Toml::file("discor.toml"))
        .merge(Env::raw())
        .extract()?;

    framework::build(&config.discord_token, config.guild_id)
        .run()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
        exit(1);
    }
}
