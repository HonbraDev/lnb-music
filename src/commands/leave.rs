use super::{base_embed, shared::get_conn, Context, Result};

/// Leave the current voice channel
#[poise::command(slash_command)]
pub(super) async fn leave(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await?;

    get_conn(&ctx).await?.lock().await.leave().await?;

    ctx.send(|r| {
        r.embed(|e| {
            base_embed(e)
                .title("Left the channel")
                .description("Hear you next time!")
        })
    })
    .await?;

    Ok(())
}
