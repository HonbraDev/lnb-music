use std::time::Duration;

use humantime::format_duration;
use songbird::input::Metadata;

use super::{base_embed, error::NotInVoiceChannelError, shared::get_conn, Context, Result};

/// Show current track
#[poise::command(slash_command, prefix_command)]
pub async fn nowplaying(ctx: Context<'_>) -> Result<()> {
    let conn = get_conn(&ctx).await?;

    let (channel_id, track) = {
        let conn = conn.lock().await;
        (
            conn.current_channel().ok_or(NotInVoiceChannelError)?,
            conn.queue().current(),
        )
    };

    let metadata = track
        .map(|track| track.metadata().clone())
        .unwrap_or(Metadata {
            title: Some("Silence".to_string()),
            duration: Some(Duration::MAX),
            thumbnail: Some(
                "https://i1.sndcdn.com/artworks-DIBLWve1lTY3YQUo-RmjVAg-t500x500.jpg".to_string(),
            ),
            ..Default::default()
        });

    ctx.send(|r| {
        r.embed(|e| {
            base_embed(e).title(format!("Now playing in <#{channel_id}>"));

            if let Some(title) = &metadata.title {
                e.field("Title", title, false);
            } else if let Some(track) = &metadata.track {
                e.field("Title", track, false);
            }

            if let Some(duration) = &metadata.duration {
                let formatted = if duration == &Duration::MAX {
                    "∞".to_string()
                } else {
                    format_duration(*duration).to_string()
                };

                e.field("Duration", formatted, true);
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
