use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait TpsThrottler: Send + Sync {
    async fn get_key(&self) -> Option<String>;
    async fn get_max_tps(&self, key: &str) -> Option<f32>;
}

#[async_trait]
impl<T: ?Sized + TpsThrottler> TpsThrottler for Option<Arc<T>> {
    async fn get_key(&self) -> Option<String> {
        if let Some(inner) = self {
            inner.get_key().await
        } else {
            None
        }
    }

    async fn get_max_tps(&self, key: &str) -> Option<f32> {
        if let Some(inner) = self {
            inner.get_max_tps(key).await
        } else {
            None
        }
    }
}
