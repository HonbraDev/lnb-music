mod join;
mod leave;
mod play;
mod shared;
mod stop;

use songbird::{error::JoinError as SongbirdJoinError, input::error::Error as SongbirdInputError};
use thiserror::Error;

use self::shared::{GetConnError, JoinChannelError};
use crate::framework;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Serenity(#[from] serenity::Error),

    #[error(transparent)]
    GetConn(#[from] GetConnError),

    #[error(transparent)]
    JoinChannel(#[from] JoinChannelError),

    #[error(transparent)]
    SongbirdJoin(#[from] SongbirdJoinError),

    #[error("Input: {0}")]
    SongbirdInput(#[from] SongbirdInputError),
}

pub type Context<'a> = poise::Context<'a, framework::Data, Error>;
pub type Result<R> = core::result::Result<R, Error>;

pub fn get() -> Vec<poise::Command<framework::Data, Error>> {
    vec![join::join(), leave::leave(), play::play(), stop::stop()]
}
