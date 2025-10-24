mod builder;

use crate::api_v1::V1Req;
use crate::api_v1::V1Rsp;
use crate::client::builder::Builder;
use api_clients_core::{ApiClientResult, Executor};
use std::sync::Arc;

#[derive(Clone)]
pub struct SwapCoffeeApiClient {
    executor: Arc<Executor>,
}

impl Default for SwapCoffeeApiClient {
    fn default() -> Self { SwapCoffeeApiClient::builder().build() }
}

impl SwapCoffeeApiClient {
    pub fn builder() -> Builder { Builder::new() }

    pub async fn exec_api_v1(&self, req: &V1Req) -> ApiClientResult<V1Rsp> {
        let rsp = match req {
            V1Req::Assets => V1Rsp::Assets(self.executor.exec_get("tokens").await?),
            V1Req::Pools(dexes) => V1Rsp::Pools(self.executor.exec_get_extra("pools", dexes, &[]).await?),
        };
        Ok(rsp)
    }
}
