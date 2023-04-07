use super::{shared::get_conn, Context, Result};

/// Stop the playback of audio
#[poise::command(slash_command)]
pub(super) async fn stop(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await?;
    get_conn(&ctx).await?.lock().await.stop();
    ctx.say("Stopped").await?;
    Ok(())
}
