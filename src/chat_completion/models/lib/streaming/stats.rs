use std::{
    ops::Add,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct StreamingChatCompletionStats {
    pub requested: Instant,
    pub chunks: Vec<StreamingChatCompletionStatsChunk>,
    pub input_tokens_is_estimate: bool,
    pub input_tokens: u32,
    pub output_tokens_is_estimate: bool,
    pub output_tokens: u32,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StreamingChatCompletionStatsChunk {
    pub offset: Duration,
    pub tokens: u32,
}

impl StreamingChatCompletionStatsChunk {
    pub fn tps(&self) -> f32 {
        let secs = self.offset.as_secs_f32();

        if secs == 0.0 {
            secs
        } else {
            self.tokens as f32 / secs
        }
    }
}

impl Add for StreamingChatCompletionStatsChunk {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            offset: self.offset + rhs.offset,
            tokens: self.tokens + rhs.tokens,
        }
    }
}

impl StreamingChatCompletionStats {
    pub fn latency(&self) -> Option<Duration> {
        self.chunks.first().map(|x| x.offset)
    }

    pub fn tps_avg(&self) -> Option<f32> {
        if self.chunks.is_empty() {
            None
        } else {
            Some(
                self.chunks
                    .iter()
                    .fold(StreamingChatCompletionStatsChunk::default(), |p, c| p + *c)
                    .tps(),
            )
        }
    }
}
