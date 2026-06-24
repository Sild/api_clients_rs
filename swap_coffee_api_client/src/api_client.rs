mod builder;

use crate::api_client::builder::Builder;
use crate::api_v1::V1Request;
use crate::api_v1::V1Response;
use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub const DEFAULT_API_V1_URL: &str = "https://backend.swap.coffee/v1";

#[derive(Clone)]
pub struct SwapCoffeeApiClient {
    executor: Arc<Executor>,
}

impl SwapCoffeeApiClient {
    pub fn builder() -> Builder { Builder::new() }

    pub async fn exec_api_v1<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<V1Response>
    where
        REQUEST: Into<V1Request>,
    {
        let request = request.into();
        let response = match &request {
            V1Request::Assets => V1Response::Assets(self.executor.exec_get("tokens").await?),
            V1Request::Pools(dexes) => V1Response::Pools(self.executor.exec_get_extra("pools", dexes, &[]).await?),
        };
        Ok(response)
    }
}
