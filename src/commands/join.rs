use super::{shared::join_channel, Context, Result};

/// Join your current voice channel
#[poise::command(slash_command)]
pub(super) async fn join(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await?;
    let (_, channel_id, _) = join_channel(&ctx).await?;
    ctx.say(format!("Joined <#{channel_id}>")).await?;
    Ok(())
}
