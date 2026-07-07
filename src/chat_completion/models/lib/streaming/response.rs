use crate::{
    chat_completion::models::{
        api::response::streaming::StreamingChatCompletionChunk,
        lib::streaming::stats::StreamingChatCompletionStats,
    },
    error::Error,
};

use std::pin::Pin;
use tokio_stream::Stream;

pub enum StreamingChatCompletionEvent {
    Done {
        stats: StreamingChatCompletionStats,
    },
    Chunk {
        chunk: StreamingChatCompletionChunk,
    },
    ChunkError {
        stats: StreamingChatCompletionStats,
        chunk: StreamingChatCompletionChunk,
    },
}

pub type StreamingChatCompletionResponse =
    Pin<Box<dyn Stream<Item = Result<StreamingChatCompletionEvent, Error>>>>;
