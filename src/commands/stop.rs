use super::{base_embed, shared::leave_channel, Context, Result};

/// Stop the playback
#[poise::command(slash_command, prefix_command)]
pub async fn stop(ctx: Context<'_>) -> Result<()> {
    leave_channel(&ctx).await?;

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
