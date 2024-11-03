use crate::client::config::StonfiApiClientConfig;
use crate::client::dex_client::DexClient;
use crate::client::executor::Executor;
use std::sync::Arc;

pub struct StonfiApiClient {
    pub dex: DexClient,
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
            dex: DexClient::new(executor),
        }
    }
}
