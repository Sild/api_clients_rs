use crate::api_v1::types::{Asset, Pool, Router};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetAssetsRsp {
    pub asset_list: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
pub struct GetAssetRsp {
    pub asset: Asset,
}

#[derive(Deserialize, Debug)]
pub struct GetPoolsRsp {
    pub pool_list: Vec<Pool>,
}

#[derive(Deserialize, Debug)]
pub struct GetPoolRsp {
    pub pool: Pool,
}

#[derive(Deserialize, Debug)]
pub struct GetRoutersRsp {
    pub router_list: Vec<Router>,
}

#[derive(Deserialize, Debug)]
pub struct GetRouterRsp {
    pub router: Router,
}

#[derive(Deserialize, Debug)]
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
