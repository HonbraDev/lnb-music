use std::sync::Arc;

use serenity::model::id::{ChannelId, GuildId};
use songbird::Call;
use tokio::sync::Mutex;

use super::{
    error::{NoSongbirdError, NoVoiceChannelIdError, NotInGuildError, NotInVoiceChannelError},
    Context, Result,
};

pub async fn get_conn(ctx: &Context<'_>) -> Result<Arc<Mutex<Call>>> {
    let guild = ctx.guild().ok_or(NotInGuildError)?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;

    let conn = manager.get(guild.id).ok_or(NotInVoiceChannelError)?;

    Ok(conn)
}

pub async fn join_channel(ctx: &Context<'_>) -> Result<(GuildId, ChannelId, Arc<Mutex<Call>>)> {
    let guild = ctx.guild().ok_or(NotInGuildError)?;

    let channel = guild
        .voice_states
        .get(&ctx.author().id)
        .ok_or(NotInVoiceChannelError)?;

    let channel_id = channel.channel_id.ok_or(NoVoiceChannelIdError)?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;

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
