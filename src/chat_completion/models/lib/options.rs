use reqwest::{Client, IntoUrl, Url};
use std::fmt::Display;
use stefans_utils::{prelude::AsClone, secret::Secret};

#[derive(Default)]
pub struct ChatCompletionOptions {
    pub client: Option<Client>,
    pub url: Option<Url>,
    pub bearer_token: Option<Secret<String>>,
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

    pub fn with_url(mut self, url: impl IntoUrl) -> Self {
        self.url = Some(url.into_url().unwrap());
        self
    }

    pub fn with_base_url(mut self, base_url: impl IntoUrl) -> Self {
        let mut url = base_url.into_url().unwrap();
        url.set_path("v1/chat/completions");
        self.url = Some(url);
        self
    }

    pub fn with_default_url(mut self) -> Self {
        self.url = None;
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
}
