use derive_more::From;
use derive_setters::Setters;
use serde_derive::Serialize;

#[derive(Clone, From)]
#[non_exhaustive]
pub enum V1Request {
    #[from(skip)]
    Assets,
    AssetsQuery(AssetsQueryParams),
    #[from(skip)]
    AssetsSearch(TODO),
    #[from(skip)]
    Asset(String),
    Farms(FarmsParams),
    #[from(skip)]
    Farm(String),
    #[from(skip)]
    FarmByPool(String),
    #[from(skip)]
    Markets,
    #[from(skip)]
    PoolQuery(TODO),
    Pools(PoolsParams),
    PoolsByMarket(PoolsByMarketParams),
    #[from(skip)]
    Pool(String),
    #[from(skip)]
    ReverseSwapSimulate(TODO),
    #[from(skip)]
    Router(String),
    Routers(RoutersParams),
    SwapSimulate(SwapSimulateParams),
    #[from(skip)]
    SwapStatus(TODO),
    TransactionQuery(TransactionQueryParams),
    #[from(skip)]
    TransactionActionTree(String),
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct TODO {}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetsQueryParams {
    pub condition: Option<String>,
    pub unconditional_assets: Vec<String>,
    pub wallet_address: Option<String>,
}

impl AssetsQueryParams {
    pub fn new() -> Self { Self::default() }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct SwapSimulateParams {
    pub offer_address: String,
    pub ask_address: String,
    pub units: String,
    pub slippage_tolerance: String,
    pub referral_address: Option<String>,
    pub referral_fee: Option<String>,
    pub dex_v2: Option<bool>,
}

impl SwapSimulateParams {
    pub fn new(
        offer_address: impl Into<String>,
        ask_address: impl Into<String>,
        units: impl Into<String>,
        slippage_tolerance: impl Into<String>,
    ) -> Self {
        Self {
            offer_address: offer_address.into(),
            ask_address: ask_address.into(),
            units: units.into(),
            slippage_tolerance: slippage_tolerance.into(),
            referral_address: None,
            referral_fee: None,
            dex_v2: Some(true),
        }
    }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct PoolsParams {
    pub dex_v2: bool,
}

impl Default for PoolsParams {
    fn default() -> Self { Self { dex_v2: true } }
}

impl PoolsParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct RoutersParams {
    pub dex_v2: bool,
}

impl Default for RoutersParams {
    fn default() -> Self { Self { dex_v2: true } }
}

impl RoutersParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct FarmsParams {
    pub dex_v2: bool,
}

impl Default for FarmsParams {
    fn default() -> Self { Self { dex_v2: true } }
}

impl FarmsParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct PoolsByMarketParams {
    pub asset0_address: String,
    pub asset1_address: String,
}

impl PoolsByMarketParams {
    pub fn new(asset0_address: impl Into<String>, asset1_address: impl Into<String>) -> Self {
        Self {
            asset0_address: asset0_address.into(),
            asset1_address: asset1_address.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TransactionQueryParams {
    pub wallet_address: Option<String>,
    pub query_id: Option<u64>,
    pub min_tx_timestamp: Option<String>,
    pub ext_msg_hash: Option<String>,
}

impl TransactionQueryParams {
    pub fn new() -> Self { Self::default() }
}

impl From<&V1Request> for V1Request {
    fn from(request: &V1Request) -> Self { request.clone() }
}
