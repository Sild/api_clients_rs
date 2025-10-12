mod builder;

use crate::api_v2::req::V2Req;
use crate::api_v2::rsp::V2Rsp;
use crate::client::builder::Builder;
use api_clients_core::{ApiClientResult, Executor};
use std::sync::Arc;

#[derive(Clone)]
pub struct DedustApiClient {
    executor: Arc<Executor>,
}

impl DedustApiClient {
    pub fn builder() -> Builder { Default::default() }

    pub async fn v2_exec(&self, req: &V2Req) -> ApiClientResult<V2Rsp> {
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
