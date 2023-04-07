use std::sync::Arc;

use serenity::model::id::{ChannelId, GuildId};
use songbird::{error::JoinError, Call};
use thiserror::Error;
use tokio::sync::Mutex;

use super::Context;

#[derive(Debug, Error)]
pub enum GetConnError {
    #[error("Songbird hasn't been initialized (this is a bug)")]
    NoSongbird,

    #[error("Not in a guild")]
    NotInGuild,

    #[error("Not in a voice channel")]
    NotInVoiceChannel,
}

pub(super) async fn get_conn(ctx: &Context<'_>) -> Result<Arc<Mutex<Call>>, GetConnError> {
    let guild = ctx.guild().ok_or(GetConnError::NotInGuild)?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(GetConnError::NoSongbird)?;

    let conn = manager
        .get(guild.id)
        .ok_or(GetConnError::NotInVoiceChannel)?;

    Ok(conn)
}

#[derive(Debug, Error)]
pub enum JoinChannelError {
    #[error("Not in a guild")]
    NotInGuild,

    #[error("Not in a voice channel")]
    NotInVoiceChannel,

    #[error("Voice channel has no ID")]
    VoiceChannelNoId,

    #[error("Songbird hasn't been initialized (this is a bug)")]
    NoSongbird,

    #[error("Failed to join the channel: {0}")]
    JoinChannel(#[from] JoinError),
}

pub(super) async fn join_channel(
    ctx: &Context<'_>,
) -> Result<(GuildId, ChannelId, Arc<Mutex<Call>>), JoinChannelError> {
    let guild = ctx.guild().ok_or(JoinChannelError::NotInGuild)?;

    let channel = guild
        .voice_states
        .get(&ctx.author().id)
        .ok_or(JoinChannelError::NotInVoiceChannel)?;

    let channel_id = channel
        .channel_id
        .ok_or(JoinChannelError::VoiceChannelNoId)?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(JoinChannelError::NoSongbird)?;

    let (conn, join_result) = manager.join(guild.id, channel_id).await;
    join_result?;

    {
        let mut conn = conn.lock().await;
        if !conn.is_deaf() {
            conn.deafen(true).await?;
        }
    }

    Ok((guild.id, channel_id, conn))
}
