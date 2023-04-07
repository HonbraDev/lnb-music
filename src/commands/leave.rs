use super::{base_embed, error::NotInVoiceChannelError, shared::get_conn, Context, Result};

/// Leave the current voice channel
#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await?;

    let channel_id = {
        let conn = get_conn(&ctx).await?;
        let mut conn = conn.lock().await;
        let current_channel = conn.current_channel().ok_or(NotInVoiceChannelError)?;
        conn.leave().await?;
        current_channel
    };

    ctx.send(|r| {
        r.embed(|e| {
            base_embed(e)
                .title(format!("Left <#{channel_id}>"))
                .description("Hear you next time!")
        })
    })
    .await?;

    Ok(())
}
