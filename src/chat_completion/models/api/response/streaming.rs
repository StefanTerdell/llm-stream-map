use indexmap::IndexMap;
use serde_json::Value;

use crate::chat_completion::models::api::{
    common::ChatCompletionUsage,
    response::common::{ChatCompletionResponseMessage, CommonChatCompletionChoice},
};

#[derive(..ApiModel)]
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

#[derive(..ApiModel)]
pub struct StreamingChatCompletionChoice {
    pub delta: ChatCompletionResponseMessage,
    #[serde(flatten)]
    pub common: CommonChatCompletionChoice,
}

#[derive(..ApiModel)]
pub struct StreamingChatCompletionToolCall {
    pub function: StreamingChatCompletionResponseFunctionToolCall,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}

#[derive(..ApiModel)]
pub struct StreamingChatCompletionResponseFunctionToolCall {
    pub name: String,
    pub arguments: String,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}
