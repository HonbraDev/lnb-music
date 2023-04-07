use humantime::format_duration;
use url::Url;

use super::{base_embed, shared::join_channel, Context, Result};

/// Play audio from a URL in your current voice channel
#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "The URL of the audio"] url: Url,
) -> Result<()> {
    ctx.defer().await?;

    let (_, channel_id, conn) = join_channel(&ctx).await?;

    let metadata = {
        let mut conn = conn.lock().await;

        let source = songbird::ytdl(&url).await?;
        let metadata = source.metadata.clone();

        conn.stop();
        conn.play_source(source);

        metadata
    };

    ctx.send(|r| {
        r.embed(|e| {
            base_embed(e).title(format!("Playing audio in <#{channel_id}>"));

            if let Some(title) = &metadata.title {
                e.field("Title", title, false);
            } else if let Some(track) = &metadata.track {
                e.field("Title", track, false);
            }

            if let Some(duration) = &metadata.duration {
                e.field("Duration", format_duration(duration.clone()), true);
            }

            if let Some(source_url) = &metadata.source_url {
                e.field("Source", format!("[Open original]({source_url})"), true);
            }

            if let Some(thumbnail) = &metadata.thumbnail {
                e.thumbnail(thumbnail);
            }

            e
        })
    })
    .await?;

    Ok(())
}
