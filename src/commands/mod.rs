mod error;
mod join;
mod leave;
mod play;
mod shared;
mod stop;

use poise::Command;
use serenity::{builder::CreateEmbed, utils::Color};

pub use self::error::Error;
use crate::framework::Data;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Result<R> = core::result::Result<R, Error>;

pub fn get() -> Vec<Command<Data, Error>> {
    vec![join::join(), leave::leave(), play::play(), stop::stop()]
}

pub fn base_embed(e: &mut CreateEmbed) -> &mut CreateEmbed {
    e.color(Color::BLURPLE)
}

pub fn base_embed_error(e: &mut CreateEmbed) -> &mut CreateEmbed {
    base_embed(e).color(Color::RED).title("Error")
}
