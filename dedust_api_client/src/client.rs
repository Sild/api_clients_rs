mod builder;

use crate::api_v2::V2Req;
use crate::api_v2::V2Rsp;
use crate::client::builder::Builder;
use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub const DEFAULT_API_V2_URL: &str = "https://api.dedust.io/v2";

#[derive(Clone)]
pub struct DedustApiClient {
    executor: Arc<Executor>,
}

impl DedustApiClient {
    pub fn builder() -> Builder { Builder::new() }

    pub async fn exec_api_v2(&self, req: &V2Req) -> ApiClientsResult<V2Rsp> {
        let rsp = match req {
            V2Req::Assets => V2Rsp::Assets(self.executor.exec_get("assets").await?),
            V2Req::Pools => V2Rsp::Pools(self.executor.exec_get("pools").await?),
            V2Req::PoolsLite => V2Rsp::PoolsLite(self.executor.exec_get("pools-lite").await?),
            V2Req::PoolTrades(pool_addr) => {
                let path = format!("pools/{}/trades", pool_addr);
                V2Rsp::PoolTrades(self.executor.exec_get(&path).await?)
            }
            V2Req::RoutingPlan(params) => {
                V2Rsp::RoutingPlan(self.executor.exec_post_body("routing/plan", params, &[]).await?)
            }
        };
        Ok(rsp)
    }
}
