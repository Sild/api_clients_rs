use crate::v1::response::V1Response;
use crate::v1::V1Request;
use api_clients_core::{ApiClientsError, ApiClientsResult, Executor};
use std::sync::Arc;

#[non_exhaustive]
pub struct V1Client {
    executor: Arc<Executor>,
}

impl V1Client {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    #[rustfmt::skip]
    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<V1Response>
    where
        REQUEST: Into<V1Request>,
    {
        let request = request.into();
        let response = match &request {
            V1Request::Assets => {
                V1Response::Assets(self.executor.exec_get("assets").await?)
            },
            V1Request::AssetsQuery(params) => {
                V1Response::AssetsQuery(self.executor.exec_post_body("assets/query", params, &[]).await?)
            },
            V1Request::Asset(addr) => {
                V1Response::Asset(self.executor.exec_get(&format!("assets/{addr}")).await?)
            },
            V1Request::Farms(params) => {
                V1Response::Farms(self.executor.exec_get_extra("farms", params, &[]).await?)
            },
            V1Request::Farm(addr) => {
                V1Response::Farm(self.executor.exec_get(&format!("farms/{addr}")).await?)
            },
            V1Request::Pool(addr) => {
                V1Response::Pool(self.executor.exec_get(&format!("pools/{addr}")).await?)
            },
            V1Request::Pools(params) => {
                V1Response::Pools(self.executor.exec_get_extra("pools", params, &[]).await?)
            },
            V1Request::PoolsByMarket(params) => {
                let path = format!("pools/by_market/{}/{}", params.asset0_address, params.asset1_address);
                V1Response::Pools(self.executor.exec_get(&path).await?)
            }
            V1Request::Routers(params) => {
                V1Response::Routers(self.executor.exec_get_extra("routers", params, &[]).await?)
            },
            V1Request::Router(addr) => {
                V1Response::Router(self.executor.exec_get(&format!("routers/{addr}")).await?)
            },
            V1Request::SwapSimulate(params) => {
                V1Response::SwapSimulate(self.executor.exec_post_qs("swap/simulate", params, &[]).await?)
            },
            V1Request::TransactionQuery(params) => {
                V1Response::TransactionQuery(self.executor.exec_get_extra("transactions/query", params, &[]).await?)
            },
            V1Request::TransactionActionTree(hash) => {
                V1Response::TransactionActionTree(self.executor.exec_get(&format!("transactions/{hash}/action_tree")).await?)
            },
            V1Request::AssetsSearch(_) => return unsupported_request("AssetsSearch"),
            V1Request::FarmByPool(_) => return unsupported_request("FarmByPool"),
            V1Request::Markets => return unsupported_request("Markets"),
            V1Request::PoolQuery(_) => return unsupported_request("PoolQuery"),
            V1Request::ReverseSwapSimulate(_) => return unsupported_request("ReverseSwapSimulate"),
            V1Request::SwapStatus(_) => return unsupported_request("SwapStatus"),
        };
        Ok(response)
    }
}

fn unsupported_request<T>(variant: &str) -> ApiClientsResult<T> {
    Err(ApiClientsError::InvalidArgs(format!("STON.fi V1 request variant {variant} is not implemented")))
}
