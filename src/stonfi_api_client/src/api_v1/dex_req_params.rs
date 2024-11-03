use serde_derive::Serialize;

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct SwapSimulateParams {
    pub offer_address: String,
    pub ask_address: String,
    pub units: String,
    pub slippage_tolerance: String,
    pub referral_address: Option<String>,
    pub referral_fee: Option<String>,
    pub dex_v2: Option<bool>,
}

#[derive(Serialize)]
pub(crate) struct PoolsParams {
    pub dex_v2: bool,
}

#[derive(Serialize)]
pub(crate) struct RoutersParams {
    pub dex_v2: bool,
}
