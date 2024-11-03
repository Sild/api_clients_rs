use crate::api_v2::types::{Pool, PoolLite};
use crate::client::executor::Executor;
use crate::client::config::DedustApiClientConfig;

pub struct DedustApiClient {
    executor: Executor,
}

impl Default for DedustApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl DedustApiClient {
    pub fn new() -> Self {
        let config = DedustApiClientConfig::default();
        Self::new_with_config(config)
    }

    pub fn new_with_config(config: DedustApiClientConfig) -> Self {
        let executor = Executor::new(&config.api_url, config.retry_count);
        Self { executor }
    }

    pub async fn get_pools(&self) -> anyhow::Result<Vec<Pool>> {
        self.executor.exec_get("pools").await
    }

    pub async fn get_pools_lite(&self) -> anyhow::Result<Vec<PoolLite>> {
        self.executor.exec_get("pools-lite").await
    }
}
