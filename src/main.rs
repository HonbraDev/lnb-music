mod commands;
mod framework;

use async_trait::async_trait;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use poise::serenity_prelude::EventHandler;
use serde::Deserialize;
use serenity::{
    client::Context,
    model::{gateway::Ready, id::GuildId},
};
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

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: Config = Figment::new()
        .merge(Toml::file("discor.toml"))
        .merge(Env::raw())
        .extract()?;

    framework::build(&config.discord_token, config.guild_id)
        .run()
        .await?;

    Ok(())
}
