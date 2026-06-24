use crate::v1::response::V1Response;
use crate::v1::V1Request;
use api_clients_core::{ApiClientsResult, Executor};
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
            V1Request::AssetsSearch(params) => {
                V1Response::AssetsSearch(self.executor.exec_post_qs("assets/search", params, &[]).await?)
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
            V1Request::FarmByPool(params) => {
                V1Response::FarmByPool(self.executor.exec_get(&format!("farms/by_pool/{}", params.pool_address)).await?)
            },
            V1Request::Markets(params) => {
                V1Response::Markets(self.executor.exec_get_extra("markets", params, &[]).await?)
            },
            V1Request::Pool(addr) => {
                V1Response::Pool(self.executor.exec_get(&format!("pools/{addr}")).await?)
            },
            V1Request::PoolQuery(params) => {
                V1Response::PoolQuery(self.executor.exec_post_body("pools/query", params, &[]).await?)
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
            V1Request::ReverseSwapSimulate(params) => {
                V1Response::ReverseSwapSimulate(self.executor.exec_post_qs("reverse_swap/simulate", params, &[]).await?)
            },
            V1Request::SwapStatus(params) => {
                V1Response::SwapStatus(self.executor.exec_get_extra("swap/status", params, &[]).await?)
            },
            V1Request::StatsDex(params) => {
                V1Response::StatsDex(self.executor.exec_get_extra("stats/dex", params, &[]).await?)
            },
            V1Request::StatsFeeAccruals(params) => {
                V1Response::StatsFeeAccruals(self.executor.exec_get_extra("stats/fee_accruals", params, &[]).await?)
            },
            V1Request::StatsFeeWithdrawals(params) => {
                V1Response::StatsFeeWithdrawals(self.executor.exec_get_extra("stats/fee_withdrawals", params, &[]).await?)
            },
            V1Request::StatsFees(params) => {
                V1Response::StatsFees(self.executor.exec_get_extra("stats/fees", params, &[]).await?)
            },
            V1Request::StatsOperations(params) => {
                V1Response::StatsOperations(self.executor.exec_get_extra("stats/operations", params, &[]).await?)
            },
            V1Request::StatsPool(params) => {
                V1Response::StatsPool(self.executor.exec_get_extra("stats/pool", params, &[]).await?)
            },
            V1Request::StatsStaking => {
                V1Response::StatsStaking(self.executor.exec_get("stats/staking").await?)
            },
            V1Request::TransactionQuery(params) => {
                V1Response::TransactionQuery(self.executor.exec_get_extra("transactions/query", params, &[]).await?)
            },
            V1Request::TransactionActionTree(hash) => {
                V1Response::TransactionActionTree(self.executor.exec_get(&format!("transactions/{hash}/action_tree")).await?)
            },
        };
        Ok(response)
    }
}
