use std::ops::Add;

use indexmap::IndexMap;
use serde_json::Value;

use crate::traits::or_merge::OrMerge;

#[derive(..ApiModel)]
pub struct CommonChatCompletionChoice {
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<IndexMap<String, Vec<Value>>>,
    #[serde(flatten)]
    pub additional_properties: IndexMap<String, Value>,
}

impl Add for CommonChatCompletionChoice {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            index: rhs.index,
            logprobs: self.logprobs.or_merge(rhs.logprobs),
            additional_properties: self
                .additional_properties
                .into_iter()
                .chain(rhs.additional_properties)
                .collect(),
        }
    }
}
