use crate::api_v1::types::{Asset, Pool, Router};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum V1DexRsp {
    Assets(AssetsRsp),
    Asset(AssetRsp),
    Pools(PoolsRsp),
    Pool(PoolRsp),
    Routers(RoutersRsp),
    Router(RouterRsp),
    SwapSimulate(SwapSimulateRsp),
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetsRsp {
    pub asset_list: Vec<Asset>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetRsp {
    pub asset: Asset,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PoolsRsp {
    pub pool_list: Vec<Pool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PoolRsp {
    pub pool: Pool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RoutersRsp {
    pub router_list: Vec<Router>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RouterRsp {
    pub router: Router,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SwapSimulateRsp {
    pub ask_address: String,
    pub ask_jetton_wallet: String,
    pub ask_units: String,
    pub fee_address: String,
    pub fee_percent: String,
    pub fee_units: String,
    pub min_ask_units: String,
    pub offer_address: String,
    pub offer_jetton_wallet: String,
    pub offer_units: String,
    pub pool_address: String,
    pub price_impact: String,
    pub router_address: String,
    pub slippage_tolerance: String,
    pub swap_rate: String,
}
