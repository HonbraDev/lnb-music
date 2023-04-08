use super::{base_embed, Context, Result};

#[poise::command(slash_command, prefix_command, discard_spare_arguments)]
pub async fn help(ctx: Context<'_>) -> Result<()> {
    ctx.send(|r| {
        r.embed(|e| {
            base_embed(e)
                .title("Help")
                .description("There is no escape.")
        })
    })
    .await?;

    Ok(())
}
