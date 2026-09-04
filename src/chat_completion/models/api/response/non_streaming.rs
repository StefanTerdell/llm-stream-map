use crate::chat_completion::models::{
    api::{
        common::{ChatCompletionMessage, ChatCompletionUsage},
        response::{common::CommonChatCompletionChoice, streaming::StreamingChatCompletionChoice},
    },
    lib::common::stats::ChatCompletionStats,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Add;
use stefans_utils::literal_str;

pub struct NonStreamingChatCompletionResponse {
    pub body: NonStreamingChatCompletionResponseBody,
    pub stats: Option<ChatCompletionStats>,
    pub error: Option<Value>,
}

literal_str!(ChatCompletionObjectLabel = "chat.completion");

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NonStreamingChatCompletionResponseBody {
    pub object: ChatCompletionObjectLabel,
    pub model: String,
    pub choices: Vec<NonStreamingChatCompletionChoice>,
    pub usage: ChatCompletionUsage,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NonStreamingChatCompletionChoice {
    pub message: ChatCompletionMessage,
    #[serde(flatten)]
    pub common: CommonChatCompletionChoice,
}

impl From<StreamingChatCompletionChoice> for NonStreamingChatCompletionChoice {
    fn from(value: StreamingChatCompletionChoice) -> Self {
        Self {
            message: value.delta,
            common: value.common,
        }
    }
}

impl Add for NonStreamingChatCompletionChoice {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            message: self.message + rhs.message,
            common: self.common + rhs.common,
        }
    }
}

impl Add<StreamingChatCompletionChoice> for NonStreamingChatCompletionChoice {
    type Output = Self;

    fn add(self, rhs: StreamingChatCompletionChoice) -> Self::Output {
        self + Self::from(rhs)
    }
}
