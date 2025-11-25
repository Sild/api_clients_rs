use crate::v1::rsp::V1Rsp;
use crate::v1::V1Req;
use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub struct V1Client {
    executor: Arc<Executor>,
}

impl V1Client {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    #[rustfmt::skip]
    pub async fn exec(&self, req: &V1Req) -> ApiClientsResult<V1Rsp> {
        let rsp = match req {
            V1Req::Assets => {
                V1Rsp::Assets(self.executor.exec_get("assets").await?)
            },
            V1Req::AssetsQuery(params) => {
                V1Rsp::AssetsQuery(self.executor.exec_post_body("assets/query", &params, &[]).await?)
            },
            V1Req::Asset(addr) => {
                V1Rsp::Asset(self.executor.exec_get(&format!("assets/{addr}")).await?)
            },
            V1Req::Farms(params) => {
                V1Rsp::Farms(self.executor.exec_get_extra("farms", &params, &[]).await?)
            },
            V1Req::Farm(addr) => {
                V1Rsp::Farm(self.executor.exec_get(&format!("farms/{addr}")).await?)
            },
            V1Req::Pool(addr) => {
                V1Rsp::Pool(self.executor.exec_get(&format!("pools/{addr}")).await?)
            },
            V1Req::Pools(params) => {
                V1Rsp::Pools(self.executor.exec_get_extra("pools", &params, &[]).await?)
            },
            V1Req::PoolsByMarket(params) => {
                let path = format!("pools/by_market/{}/{}", params.asset0_address, params.asset1_address);
                V1Rsp::Pools(self.executor.exec_get(&path).await?)
            }
            V1Req::Routers(params) => {
                V1Rsp::Routers(self.executor.exec_get_extra("routers", &params, &[]).await?)
            },
            V1Req::Router(addr) => {
                V1Rsp::Router(self.executor.exec_get(&format!("routers/{addr}")).await?)
            },
            V1Req::SwapSimulate(params) => {
                V1Rsp::SwapSimulate(self.executor.exec_post_qs("swap/simulate", params, &[]).await?)
            },
            V1Req::TransactionQuery(params) => {
                V1Rsp::TransactionQuery(self.executor.exec_get_extra("transactions/query", &params, &[]).await?)
            },
            V1Req::TransactionActionTree(hash) => {
                V1Rsp::TransactionActionTree(self.executor.exec_get(&format!("transactions/{hash}/action_tree")).await?)
            },
            _ => unimplemented!(),
        };
        Ok(rsp)
    }
}
