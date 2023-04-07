use super::{base_embed, shared::join_channel, Context, Result};

/// Join your current voice channel
#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await?;

    let (_, channel_id, _) = join_channel(&ctx).await?;

    ctx.send(|r| {
        r.embed(|e| {
            base_embed(e)
                .title(format!("Joined <#{channel_id}>"))
                .description("It's hearing time!")
        })
    })
    .await?;

    Ok(())
}
