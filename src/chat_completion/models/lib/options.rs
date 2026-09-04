use reqwest::Client;
use std::{fmt::Display, sync::Arc};
use stefans_utils::{as_arc::AsArc, prelude::AsClone, secret::Secret};
pub mod reasoning_content_remapping;

use crate::{
    chat_completion::models::lib::options::reasoning_content_remapping::ReasoningContentRemappingConfig,
    traits::tps_throttler::TpsThrottler,
};

#[derive(Default)]
pub struct ChatCompletionOptions {
    pub client: Option<Client>,
    pub bearer_token: Option<Secret<String>>,
    pub tps_throttler: Option<Arc<dyn TpsThrottler>>,
    pub reasoning_content_remapping: Option<ReasoningContentRemappingConfig>,
}

impl ChatCompletionOptions {
    pub fn with_client(mut self, client: impl AsClone<Client>) -> Self {
        self.client = Some(client.as_clone());
        self
    }

    pub fn with_default_client(mut self) -> Self {
        self.client = None;
        self
    }

    pub fn with_bearer_token<T: Display>(mut self, bearer_token: impl Into<Secret<T>>) -> Self {
        self.bearer_token = Some(bearer_token.into().expose_to_string().into());
        self
    }

    pub fn without_bearer_token(mut self) -> Self {
        self.bearer_token = None;
        self
    }

    pub fn with_tps_throttler<T: TpsThrottler + 'static>(
        mut self,
        tps_throttler: impl AsArc<T>,
    ) -> Self {
        self.tps_throttler = Some(tps_throttler.as_arc());
        self
    }

    pub fn without_tps_throttler(mut self) -> Self {
        self.tps_throttler = None;
        self
    }

    pub fn with_remap_reasoning(
        mut self,
        remap_reasoning: impl Into<ReasoningContentRemappingConfig>,
    ) -> Self {
        self.reasoning_content_remapping = Some(remap_reasoning.into());
        self
    }

    pub fn without_remap_reasoning(mut self) -> Self {
        self.reasoning_content_remapping = None;
        self
    }
}
