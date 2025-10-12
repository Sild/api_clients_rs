use crate::client::v1_dex::V1Dex;
use crate::client::StonfiApiClient;
use api_clients_core::Executor;
use derive_setters::Setters;
use std::sync::Arc;

const DEFAULT_API_V1_URL: &str = "https://api.ston.fi/v1";

#[derive(Setters, Debug, Default)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    api_url: Option<String>,
    retry_count: Option<u32>,
}

impl Builder {
    pub fn build(self) -> StonfiApiClient {
        let api_url = self.api_url.unwrap_or_else(|| DEFAULT_API_V1_URL.to_string());
        let retry_count = self.retry_count.unwrap_or(3);
        let executor = Arc::new(Executor::new(&api_url, retry_count));
        let v1_dex = V1Dex::new(executor);
        StonfiApiClient { v1_dex }
    }
}
