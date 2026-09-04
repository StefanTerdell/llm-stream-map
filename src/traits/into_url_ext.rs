use reqwest::IntoUrl;

use crate::error::Error;

pub trait IntoUrlExt {
    fn with_default_chat_completions_path(self) -> Result<impl IntoUrl, Error>;
}

impl<T: IntoUrl> IntoUrlExt for T {
    fn with_default_chat_completions_path(self) -> Result<impl IntoUrl, Error> {
        Ok(self.into_url()?.join("/v1/chat/completions")?)
    }
}
