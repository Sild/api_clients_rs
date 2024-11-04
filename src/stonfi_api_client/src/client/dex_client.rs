use crate::api_v1::dex_req::V1DexReq;
use crate::api_v1::dex_rsp::V1DexRsp;
use crate::client::executor::Executor;
use anyhow::Result;
use std::sync::Arc;

pub struct DexClient {
    executor: Arc<Executor>,
}

impl DexClient {
    pub(super) fn new(executor: Arc<Executor>) -> Self {
        Self { executor }
    }

    pub async fn v1_exec(&self, req: &V1DexReq) -> Result<V1DexRsp> {
        let rsp = match req {
            V1DexReq::Assets => V1DexRsp::Assets(self.executor.exec_get("assets").await?),
            V1DexReq::Asset(addr) => V1DexRsp::Asset(self.executor.exec_get(&format!("assets/{}", addr)).await?),
            V1DexReq::Pools(params) => V1DexRsp::Pools(self.executor.exec_get_with_params("pools", &params).await?),
            V1DexReq::Pool(addr) => V1DexRsp::Pool(self.executor.exec_get(&format!("pools/{}", addr)).await?),
            V1DexReq::Routers(params) => {
                V1DexRsp::Routers(self.executor.exec_get_with_params("routers", &params).await?)
            }
            V1DexReq::Router(addr) => V1DexRsp::Router(self.executor.exec_get(&format!("routers/{}", addr)).await?),
            V1DexReq::SwapSimulate(params) => {
                V1DexRsp::SwapSimulate(self.executor.exec_post("swap/simulate", params).await?)
            }
        };
        Ok(rsp)
    }
}
