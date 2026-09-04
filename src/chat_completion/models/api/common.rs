use crate::{
    chat_completion::models::lib::common::stats::ChatCompletionStats,
    traits::{or_add::OrAdd, or_merge::OrMerge},
};

use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{convert::Infallible, ops::Add, str::FromStr};
use stefans_utils::literal_str;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChatCompletionMessage {
    pub role: Option<String>,
    pub content: Option<ChatCompletionRequestMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionRequestMessageToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}

impl Add for ChatCompletionMessage {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            role: rhs.role.or(self.role),
            tool_calls: self.tool_calls.or_merge(rhs.tool_calls),
            content: self.content.or_add(rhs.content),
            reasoning_content: self.reasoning_content.or_merge(rhs.reasoning_content),
            reasoning: self.reasoning.or_merge(rhs.reasoning),
            name: rhs.name.or(self.name),
            additional_properties: self
                .additional_properties
                .into_iter()
                .chain(rhs.additional_properties)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChatCompletionRequestMessageToolCall {
    pub function: ChatCompletionRequestMessageFunctionToolCall,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChatCompletionRequestMessageFunctionToolCall {
    pub name: String,
    pub arguments: String,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContent {
    Text(String),
    Parts(Vec<ChatCompletionRequestMessageContentPart>),
}

impl core::iter::Sum for ChatCompletionRequestMessageContent {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::Parts(vec![]), |p, c| p + c)
    }
}
impl Add for ChatCompletionRequestMessageContent {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Text(text) => match rhs {
                Self::Text(rhs_text) => Self::Text(format!("{text}{rhs_text}")),
                Self::Parts(mut vec) => {
                    if vec.is_empty() {
                        Self::Text(text)
                    } else {
                        vec.insert(0, text.into());
                        Self::Parts(vec)
                    }
                }
            },
            Self::Parts(mut vec) => match rhs {
                Self::Text(text) => {
                    if vec.is_empty() {
                        Self::Text(text)
                    } else {
                        vec.push(text.into());
                        Self::Parts(vec)
                    }
                }
                Self::Parts(mut rhs_vec) => {
                    vec.append(&mut rhs_vec);
                    Self::Parts(vec)
                }
            },
        }
    }
}

literal_str!(ChatCompletionRequestMessageContentPartTextType = "text");

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPart {
    Text {
        r#type: ChatCompletionRequestMessageContentPartTextType,
        text: String,
        #[serde(flatten)]
        additional_properties: IndexMap<String, Value>,
    },
    #[serde(untagged)]
    Other(Value),
}

impl From<String> for ChatCompletionRequestMessageContentPart {
    fn from(value: String) -> Self {
        Self::Text {
            text: value,
            r#type: Default::default(),
            additional_properties: Default::default(),
        }
    }
}

impl FromStr for ChatCompletionRequestMessageContentPart {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Text {
            r#type: Default::default(),
            text: s.to_string(),
            additional_properties: Default::default(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ChatCompletionUsage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
    #[serde(flatten)]
    additional_properties: IndexMap<String, Value>,
}

impl From<&ChatCompletionStats> for ChatCompletionUsage {
    fn from(value: &ChatCompletionStats) -> Self {
        Self {
            completion_tokens: value.output_tokens,
            prompt_tokens: value.input_tokens,
            total_tokens: value.input_tokens + value.output_tokens,
            additional_properties: Default::default(),
        }
    }
}

impl ChatCompletionUsage {
    pub fn with_completion_tokens(mut self, completion_tokens: u32) -> Self {
        self.completion_tokens = completion_tokens;
        self.total_tokens = self.completion_tokens + self.prompt_tokens;
        self
    }

    pub fn with_prompt_tokens(mut self, prompt_tokens: u32) -> Self {
        self.prompt_tokens = prompt_tokens;
        self.total_tokens = self.prompt_tokens + self.completion_tokens;
        self
    }

    pub fn completion_tokens(&self) -> u32 {
        self.completion_tokens
    }

    pub fn prompt_tokens(&self) -> u32 {
        self.prompt_tokens
    }

    pub fn total_tokens(&self) -> u32 {
        self.total_tokens
    }
}
