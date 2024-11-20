use crate::api_v2::req::V2Req;
use crate::api_v2::rsp::V2Rsp;
use crate::client::config::DedustApiClientConfig;
use crate::client::executor::Executor;

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

    pub async fn v2_exec(&self, req: &V2Req) -> anyhow::Result<V2Rsp> {
        let rsp = match req {
            V2Req::Pools => V2Rsp::Pools(self.executor.exec_get("pools").await?),
            V2Req::PoolsLite => V2Rsp::PoolsLite(self.executor.exec_get("pools-lite").await?),
            V2Req::PoolTrades(pool_addr) => {
                let path = format!("pools/{}/trades", pool_addr);
                V2Rsp::PoolTrades(self.executor.exec_get(&path).await?)
            },
            V2Req::RoutingPlan(params) => V2Rsp::RoutingPlan(self.executor.exec_post("routing/plan", params).await?),
        };
        Ok(rsp)
    }
}
