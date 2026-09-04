use indexmap::IndexMap;
use serde_json::Value;

use crate::chat_completion::models::api::common::ChatCompletionMessage;

#[derive(..ApiModel)]
pub struct CommonChatCompletionRequestBody {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}
