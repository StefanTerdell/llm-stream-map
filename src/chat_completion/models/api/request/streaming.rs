use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use stefans_utils::literals::True;

use crate::chat_completion::models::api::request::{
    ChatCompletionRequestBody, common::CommonChatCompletionRequestBody,
    non_streaming::NonStreamingChatCompletionRequestBody,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StreamingChatCompletionRequestBody {
    pub stream: True,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamingChatCompletionRequestBodyStreamOptions>,
    #[serde(flatten)]
    pub common: CommonChatCompletionRequestBody,
}

impl From<StreamingChatCompletionRequestBody> for NonStreamingChatCompletionRequestBody {
    fn from(value: StreamingChatCompletionRequestBody) -> Self {
        Self {
            stream: None,
            common: value.common,
        }
    }
}

impl From<StreamingChatCompletionRequestBody> for ChatCompletionRequestBody {
    fn from(value: StreamingChatCompletionRequestBody) -> Self {
        Self::Streaming(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StreamingChatCompletionRequestBodyStreamOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}
