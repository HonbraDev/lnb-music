use tokio::time::{sleep, Duration};

use super::{base_embed, base_embed_error, Context, Result};

#[poise::command(slash_command, prefix_command)]
pub async fn order(
    ctx: Context<'_>,
    #[description = "What to order"]
    #[rest]
    what: String,
) -> Result<()> {
    let handle = ctx
        .send(|r| {
            r.embed(|e| {
                base_embed(e)
                    .title("Order product")
                    .description(format!("Searching Amazon for `{what}`..."))
            })
        })
        .await?;

    sleep(Duration::from_secs(3)).await;

    handle
        .edit(ctx, |r| {
            r.embed(|e| {
                base_embed_error(e)
                    .title("No results found")
                    .description("Please try again with a different query.")
            })
        })
        .await?;

    Ok(())
}
