use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::chat_completion::models::api::{
    common::{ChatCompletionMessage, ChatCompletionUsage},
    response::common::CommonChatCompletionChoice,
};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreamingChatCompletionChunk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<StreamingChatCompletionChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ChatCompletionUsage>,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreamingChatCompletionChoice {
    pub delta: ChatCompletionMessage,
    #[serde(flatten)]
    pub common: CommonChatCompletionChoice,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreamingChatCompletionToolCall {
    pub function: StreamingChatCompletionResponseFunctionToolCall,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreamingChatCompletionResponseFunctionToolCall {
    pub name: String,
    pub arguments: String,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}
