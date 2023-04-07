use super::{base_embed, shared::leave_channel, Context, Result};

/// Leave the current voice channel
#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await?;

    let (_, channel_id) = leave_channel(&ctx).await?;

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
