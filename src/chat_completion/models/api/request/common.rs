use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::chat_completion::models::api::common::ChatCompletionMessage;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CommonChatCompletionRequestBody {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}
