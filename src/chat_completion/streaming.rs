use crate::{
    chat_completion::models::{
        api::{
            request::streaming::{
                StreamingChatCompletionRequestBody, StreamingChatCompletionRequestBodyStreamOptions,
            },
            response::streaming::StreamingChatCompletionChunk,
        },
        lib::{
            options::ChatCompletionOptions,
            streaming::{
                response::{StreamingChatCompletionEvent, StreamingChatCompletionResponse},
                stats::{StreamingChatCompletionStats, StreamingChatCompletionStatsChunk},
            },
        },
    },
    error::Error,
    traits::estimate_tokens::EstimateTokens,
};

use async_stream::try_stream;
use reqwest_sse::{Event, EventSource};
use std::time::Instant;
use stefans_utils::prelude::AsBool;
use tokio_stream::StreamExt;

pub async fn streaming_chat_completion(
    body: impl Into<StreamingChatCompletionRequestBody>,
    options: impl Into<Option<ChatCompletionOptions>>,
) -> Result<StreamingChatCompletionResponse, Error> {
    let mut body = body.into();

    let (client, url, bearer_token) = {
        let options_opt = options.into();
        let (client_opt, url_opt, bearer_token) = match options_opt {
            Some(options) => (options.client, options.url, options.bearer_token),
            None => (None, None, None),
        };
        let client = client_opt.unwrap_or_default();
        let url = url_opt.unwrap_or(
            "https://api.openai.com/v1/chat/completions"
                .parse()
                .unwrap(),
        );

        (client, url, bearer_token)
    };

    let requested_logprobs = body.common.logprobs.as_bool();
    let requested_usage = body
        .stream_options
        .as_ref()
        .is_some_and(|so| so.include_usage.as_bool());

    body.common.logprobs = Some(true);

    match body.stream_options.as_mut() {
        Some(stream_options) => {
            stream_options.include_usage = Some(true);
        }
        None => {
            body.stream_options = Some(StreamingChatCompletionRequestBodyStreamOptions {
                include_usage: Some(true),
                additional_properties: Default::default(),
            });
        }
    }

    let mut request = client.post(url).json(&body);

    if let Some(secret) = bearer_token {
        request = request.bearer_auth(secret.expose())
    };

    let mut stats = StreamingChatCompletionStats {
        requested: Instant::now(),
        chunks: Default::default(),
        input_tokens_is_estimate: true,
        input_tokens: body.common.estimate_tokens(),
        output_tokens_is_estimate: true,
        output_tokens: 0,
        error: None,
    };

    let mut last_received = stats.requested;
    let mut stream = request.send().await?.events().await?;
    let mut partial_buffer = None;

    Ok(Box::pin(try_stream! {
        while let Some(Event { mut data, .. }) = stream.next().await.transpose()? {
            if data == "[DONE]" {
                yield StreamingChatCompletionEvent::Done { stats };
                break;
            }

            if let Some(previous) = partial_buffer.take() {
                data = format!("{previous}{data}");
            }

            let Ok(mut chunk) =
                serde_json::from_str::<StreamingChatCompletionChunk>(&data)
            else {
                partial_buffer = Some(data);
                continue;
            };

            if let Some(choices) = &mut chunk.choices {
                let now = Instant::now();
                let offset = now - last_received;
                last_received = now;

                let mut tokens = 0;

                for choice in choices {
                    if let Some(logprobs) = choice.common.logprobs.take() {
                        let count: u32 = logprobs.values().map(|v| v.len() as u32).sum();
                        stats.output_tokens_is_estimate = false;
                        stats.output_tokens += count;
                        tokens += count;

                        if requested_logprobs {
                            choice.common.logprobs = Some(logprobs);
                        }
                    } else {
                        stats.output_tokens_is_estimate = true;
                        tokens += choice.delta.estimate_tokens();
                    }
                }

                stats.chunks
                    .push(StreamingChatCompletionStatsChunk { offset, tokens })
            }

            if let Some(usage) = chunk.usage.take() {
                stats.input_tokens_is_estimate = false;
                stats.input_tokens = usage.prompt_tokens();
                stats.output_tokens_is_estimate = false;
                stats.output_tokens = usage.completion_tokens();

                if requested_usage {
                    chunk.usage = Some(usage);
                }
            }

            if let Some(error) = chunk.error.as_ref().map(|e| e.to_string()).filter(|e| !e.is_empty()) {
                stats.error = Some(error);
                yield StreamingChatCompletionEvent::ChunkError { chunk, stats };
                break;
            }

            yield StreamingChatCompletionEvent::Chunk { chunk };
        }
    }))
}
