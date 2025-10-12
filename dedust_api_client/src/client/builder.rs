use crate::client::DedustApiClient;
use api_clients_core::Executor;
use derive_setters::Setters;
use std::sync::Arc;

pub const DEFAULT_API_V2_URL: &str = "https://api.dedust.io/v2";

#[derive(Setters, Default, Debug)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    api_url: Option<String>,
    retry_count: Option<u32>,
}

impl Builder {
    pub fn build(self) -> DedustApiClient {
        let api_url = self.api_url.unwrap_or_else(|| DEFAULT_API_V2_URL.to_string());
        let retry_count = self.retry_count.unwrap_or(3);
        DedustApiClient {
            executor: Arc::new(Executor::new(&api_url, retry_count)),
        }
    }
}
