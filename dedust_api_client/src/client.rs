mod builder;

use crate::api_v2::V2Request;
use crate::api_v2::V2Response;
use crate::client::builder::Builder;
use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub const DEFAULT_API_V2_URL: &str = "https://api.dedust.io/v2";

#[derive(Clone)]
#[non_exhaustive]
pub struct DedustApiClient {
    executor: Arc<Executor>,
}

impl DedustApiClient {
    pub fn builder() -> Builder { Builder::new() }

    pub async fn exec_api_v2<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<V2Response>
    where
        REQUEST: Into<V2Request>,
    {
        let request = request.into();
        let response = match &request {
            V2Request::Assets => V2Response::Assets(self.executor.exec_get("assets").await?),
            V2Request::Pools => V2Response::Pools(self.executor.exec_get("pools").await?),
            V2Request::PoolsLite => V2Response::PoolsLite(self.executor.exec_get("pools-lite").await?),
            V2Request::PoolTrades(pool_addr) => {
                let path = format!("pools/{}/trades", pool_addr);
                V2Response::PoolTrades(self.executor.exec_get(&path).await?)
            }
            V2Request::RoutingPlan(params) => {
                V2Response::RoutingPlan(self.executor.exec_post_body("routing/plan", params, &[]).await?)
            }
        };
        Ok(response)
    }
}
