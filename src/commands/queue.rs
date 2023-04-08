use songbird::input::Metadata;

use super::{base_embed, error::NotInVoiceChannelError, shared::get_conn, Context, Result};

/// Show queued tracks
#[poise::command(slash_command, prefix_command)]
pub async fn queue(ctx: Context<'_>) -> Result<()> {
    let conn = get_conn(&ctx).await?;

    let (current_channel, queue) = {
        let conn = conn.lock().await;
        (
            conn.current_channel().ok_or(NotInVoiceChannelError)?,
            conn.queue().current_queue(),
        )
    };

    ctx.send(|r| {
        r.embed(|e| {
            base_embed(e).title(format!("Queue in <#{current_channel}>"));

            if queue.is_empty() {
                e.description("The queue is empty");
            } else {
                let body = queue
                    .iter()
                    .map(|track| display_track(track.metadata()))
                    .collect::<Vec<_>>()
                    .join("\n");

                e.description(body);

                if let Some(thumbnail) = &queue
                    .get(0)
                    .expect("Queue is not empty right after checking")
                    .metadata()
                    .thumbnail
                {
                    e.thumbnail(thumbnail);
                }
            }

            e
        })
    })
    .await?;

    Ok(())
}

fn display_track(metadata: &Metadata) -> String {
    let no_title = "<no_title>".to_string();

    let title = metadata
        .title
        .as_ref()
        .or(metadata.track.as_ref())
        .unwrap_or(&no_title);

    if let Some(source_url) = &metadata.source_url {
        format!("• [{title}]({source_url})")
    } else {
        format!("• {title}")
    }
}
