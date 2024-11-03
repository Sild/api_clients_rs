use crate::api_v1::dex_req_params::{PoolsParams, RoutersParams, SwapSimulateParams};
use crate::api_v1::dex_rsp::{
    GetAssetRsp, GetAssetsRsp, GetPoolRsp, GetPoolsRsp, GetRouterRsp, GetRoutersRsp, SwapSimulateRsp,
};
use crate::client::executor::Executor;
use anyhow::Result;
use std::sync::Arc;

pub struct DexClient {
    executor: Arc<Executor>,
}

impl DexClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self {
        Self { executor }
    }

    pub async fn get_assets(&self) -> Result<GetAssetsRsp> {
        self.executor.exec_get("assets").await
    }

    pub async fn get_asset(&self, addr: &str) -> Result<Option<GetAssetRsp>> {
        let path = format!("assets/{}", addr);
        self.executor.exec_get(&path).await
    }

    pub async fn get_pools(&self, dex_v2: bool) -> Result<GetPoolsRsp> {
        let params = PoolsParams { dex_v2 };
        self.executor.exec_get_with_params("pools", &params).await
    }

    pub async fn get_pool(&self, addr: &str) -> Result<Option<GetPoolRsp>> {
        let path = format!("pools/{}", addr);
        self.executor.exec_get(&path).await
    }

    pub async fn get_routers(&self, dex_v2: bool) -> Result<GetRoutersRsp> {
        let params = RoutersParams { dex_v2 };
        self.executor.exec_get_with_params("routers", &params).await
    }

    pub async fn get_router(&self, addr: &str) -> Result<Option<GetRouterRsp>> {
        let path = format!("routers/{}", addr);
        self.executor.exec_get(&path).await
    }

    pub async fn swap_simulate(&self, params: &SwapSimulateParams) -> Result<SwapSimulateRsp> {
        self.executor.exec_post("swap/simulate", params).await
    }
}
