use crate::client::v1_dex::V1Dex;
use crate::client::StonfiApiClient;
use api_clients_core::Executor;
use derive_setters::Setters;
use std::sync::Arc;

const DEFAULT_API_V1_URL: &str = "https://api.ston.fi/v1";

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
            api_url: DEFAULT_API_V1_URL.to_string(),
            retry_count: 3,
            executor: None,
        }
    }

    pub fn build(self) -> StonfiApiClient {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.api_url).with_retry_count(self.retry_count).build().into(),
        };

        let v1_dex = V1Dex::new(executor);
        StonfiApiClient { v1_dex }
    }
}
