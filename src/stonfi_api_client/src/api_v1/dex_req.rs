use serde_derive::Serialize;

pub enum V1DexReq {
    Assets,
    Asset(String),
    Pools(PoolsParams),
    Pool(String),
    Router(String),
    Routers(RoutersParams),
    SwapSimulate(SwapSimulateParams),
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct SwapSimulateParams {
    pub offer_address: String,
    pub ask_address: String,
    pub units: String,
    pub slippage_tolerance: String,
    pub referral_address: Option<String>,
    pub referral_fee: Option<String>,
    pub dex_v2: Option<bool>,
}

#[derive(Serialize, Clone)]
pub struct PoolsParams {
    pub dex_v2: bool,
}

#[derive(Serialize, Clone)]
pub struct RoutersParams {
    pub dex_v2: bool,
}
