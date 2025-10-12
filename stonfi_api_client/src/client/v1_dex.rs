use crate::api_v1::dex_req::V1DexReq;
use crate::api_v1::dex_rsp::V1DexRsp;
use api_clients_core::{ApiClientResult, Executor};
use std::sync::Arc;

pub struct V1Dex {
    executor: Arc<Executor>,
}

impl V1Dex {
    pub(super) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    #[rustfmt::skip]
    pub async fn exec(&self, req: &V1DexReq) -> ApiClientResult<V1DexRsp> {
        let rsp = match req {
            V1DexReq::Assets => V1DexRsp::Assets(self.executor.exec_get("assets").await?),
            V1DexReq::AssetsQuery(params) => V1DexRsp::AssetsQuery(self.executor.exec_post_qs("assets/query", &params, &[]).await?),
            V1DexReq::Asset(addr) => V1DexRsp::Asset(self.executor.exec_get(&format!("assets/{addr}")).await?),
            V1DexReq::Farms(params) => V1DexRsp::Farms(self.executor.exec_get_extra("farms", &params, &[]).await?),
            V1DexReq::Farm(addr) => V1DexRsp::Farm(self.executor.exec_get(&format!("farms/{addr}")).await?),
            V1DexReq::Pool(addr) => V1DexRsp::Pool(self.executor.exec_get(&format!("pools/{addr}")).await?),
            V1DexReq::Pools(params) => V1DexRsp::Pools(self.executor.exec_get_extra("pools", &params, &[]).await?),
            V1DexReq::PoolsByMarket(params) => {
                let path = format!("pools/by_market/{}/{}", params.asset0_address, params.asset1_address);
                V1DexRsp::Pools(self.executor.exec_get(&path).await?)
            }
            V1DexReq::Routers(params) => V1DexRsp::Routers(self.executor.exec_get_extra("routers", &params, &[]).await?),
            V1DexReq::Router(addr) => V1DexRsp::Router(self.executor.exec_get(&format!("routers/{addr}")).await?),
            V1DexReq::SwapSimulate(params) => V1DexRsp::SwapSimulate(self.executor.exec_post_qs("swap/simulate", params, &[]).await?),
            _ => unimplemented!(),
        };
        Ok(rsp)
    }
}
