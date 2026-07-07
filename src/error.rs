pub use reqwest::Error as ReqwestError;
pub use reqwest_sse::error::{
    EventError as ReqwestSseEventError, EventSourceError as ReqwestSseEventSourceError,
};
pub use serde_json::Error as SerdeJsonError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] ReqwestError),
    #[error(transparent)]
    SerdeJson(#[from] SerdeJsonError),
    #[error(transparent)]
    ReqwestSseReqwestSseEventSource(#[from] ReqwestSseEventSourceError),
    #[error(transparent)]
    ReqwestSseReqwestSseSource(#[from] ReqwestSseEventError),
}
