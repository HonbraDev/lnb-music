use super::{shared::get_conn, Context, Result};

/// Leave the current voice channel
#[poise::command(slash_command)]
pub(super) async fn leave(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await?;
    get_conn(&ctx).await?.lock().await.leave().await?;
    ctx.say("Left the channel").await?;
    Ok(())
}
