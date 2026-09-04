use std::{
    ops::Add,
    time::{Duration, Instant},
};

use crate::{chat_completion::models::api::common::ChatCompletionUsage, traits::or_add::OrAdd};

#[derive(Debug, Clone)]
pub struct ChatCompletionStats {
    pub requested: Instant,
    pub chunks: Vec<ChatCompletionChunkStats>,
    pub input_tokens_is_estimate: bool,
    pub input_tokens: u32,
    pub output_tokens_is_estimate: bool,
    pub output_tokens: u32,
    pub error: Option<String>,
}

impl ChatCompletionStats {
    pub fn as_usage(self) -> ChatCompletionUsage {
        (&self).into()
    }

    pub fn latency(&self) -> Option<Duration> {
        self.chunks.first().map(|x| x.duration)
    }

    pub fn tps_avg(&self) -> Option<f32> {
        if self.chunks.is_empty() {
            None
        } else {
            Some(
                self.chunks
                    .iter()
                    .fold(ChatCompletionChunkStats::default(), |p, c| p + *c)
                    .tps(),
            )
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ChatCompletionChunkStats {
    pub duration: Duration,
    pub tokens: u32,
    pub tps_correction_duration: Option<Duration>,
}

impl ChatCompletionChunkStats {
    pub fn tps(&self) -> f32 {
        let secs = self.duration.as_secs_f32();

        if secs == 0.0 {
            secs
        } else {
            self.tokens as f32 / secs
        }
    }
}

impl Add for ChatCompletionChunkStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            duration: self.duration + rhs.duration,
            tokens: self.tokens + rhs.tokens,
            tps_correction_duration: self
                .tps_correction_duration
                .or_add(rhs.tps_correction_duration),
        }
    }
}
