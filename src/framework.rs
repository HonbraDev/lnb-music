use poise::{FrameworkBuilder, FrameworkError, PrefixFrameworkOptions};
use serenity::model::{gateway::GatewayIntents, id::GuildId};
use songbird::SerenityInit;

use crate::commands::{self, base_embed_error, Error as CommandError};

pub struct Data {}

pub fn build(token: &str, guild_id: GuildId) -> FrameworkBuilder<Data, CommandError> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get(),
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        FrameworkError::Command { error, ctx } => {
                            let _ = ctx
                                .send(|r| {
                                    r.embed(|e| {
                                        base_embed_error(e).description(error.to_string())
                                    })
                                })
                                .await;
                        }
                        error => {
                            let _ = poise::builtins::on_error(error).await;
                        }
                    }
                })
            },
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("alexa".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .intents(
            /* GatewayIntents::GUILDS
            | GatewayIntents::GUILD_VOICE_STATES
            | GatewayIntents::MESSAGE_CONTENT, */
            GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
        )
        .token(token)
        .client_settings(|c| c.register_songbird())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;

                Ok(Data {})
            })
        })
}

//    |\__/,|   (`\
//  _.|o o  |_   ) )
// -(((---(((--------
