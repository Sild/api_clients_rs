use crate::client::config::StonfiApiClientConfig;
use crate::client::v1_dex::V1Dex;
use std::sync::Arc;
use api_clients_core::Executor;

pub struct StonfiApiClient {
    pub v1_dex: V1Dex,
}

impl Default for StonfiApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl StonfiApiClient {
    pub fn new() -> Self {
        let config = StonfiApiClientConfig::default();
        Self::new_with_config(config)
    }

    pub fn new_with_config(config: StonfiApiClientConfig) -> Self {
        let executor = Arc::new(Executor::new(&config.api_url, config.retry_count));
        Self {
            v1_dex: V1Dex::new(executor),
        }
    }
}
