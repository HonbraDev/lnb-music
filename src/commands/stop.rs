use super::{base_embed, shared::get_conn, Context, Result};

/// Stop the playback of audio
#[poise::command(slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await?;
    get_conn(&ctx).await?.lock().await.stop();
    ctx.send(|r| {
        r.embed(|e| {
            base_embed(e)
                .title("Stopped playing")
                .description("May your eardrums take a break.")
        })
    })
    .await?;
    Ok(())
}
