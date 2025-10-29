use crate::client::{DedustApiClient, DEFAULT_API_V2_URL};
use api_clients_core::Executor;
use derive_setters::Setters;
use std::sync::Arc;

#[derive(Setters)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    api_url: String,
    retry_count: u32,
    executor: Option<Arc<Executor>>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            api_url: DEFAULT_API_V2_URL.to_string(),
            retry_count: 3,
            executor: None,
        }
    }

    pub fn build(self) -> DedustApiClient {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.api_url).with_retry_count(self.retry_count).build().into(),
        };

        DedustApiClient { executor }
    }
}
