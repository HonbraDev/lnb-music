use url::Url;

use super::{shared::join_channel, Context, Result};

/// Play audio from a URL in your current voice channel
#[poise::command(slash_command)]
pub(super) async fn play(
    ctx: Context<'_>,
    #[description = "The URL of the audio"] url: Url,
) -> Result<()> {
    ctx.defer().await?;

    let (_, channel_id, conn) = join_channel(&ctx).await?;

    {
        let mut conn = conn.lock().await;

        let source = songbird::ytdl(&url).await?;

        conn.stop();
        conn.play_source(source);
    }

    ctx.say(format!("Playing in <#{channel_id}>")).await?;

    Ok(())
}
