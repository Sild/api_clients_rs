use derive_more::From;
use derive_setters::Setters;
use serde_derive::Serialize;

#[derive(Clone, From)]
#[non_exhaustive]
pub enum V1Request {
    #[from(skip)]
    Assets,
    AssetsQuery(AssetsQueryParams),
    AssetsSearch(AssetsSearchParams),
    #[from(skip)]
    Asset(String),
    Farms(FarmsParams),
    #[from(skip)]
    Farm(String),
    FarmByPool(FarmByPoolParams),
    Markets(MarketsParams),
    PoolQuery(PoolQueryParams),
    Pools(PoolsParams),
    PoolsByMarket(PoolsByMarketParams),
    #[from(skip)]
    Pool(String),
    ReverseSwapSimulate(ReverseSwapSimulateParams),
    #[from(skip)]
    Router(String),
    Routers(RoutersParams),
    SwapSimulate(SwapSimulateParams),
    SwapStatus(SwapStatusParams),
    StatsDex(StatsDexParams),
    StatsFeeAccruals(StatsFeeAccrualsParams),
    StatsFeeWithdrawals(StatsFeeWithdrawalsParams),
    StatsFees(StatsFeesParams),
    StatsOperations(StatsOperationsParams),
    StatsPool(StatsPoolParams),
    #[from(skip)]
    StatsStaking,
    TransactionQuery(TransactionQueryParams),
    #[from(skip)]
    TransactionActionTree(String),
}

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
pub struct AssetsSearchParams {
    pub search_string: String,
    pub condition: Option<String>,
    #[serde(rename = "unconditional_asset")]
    pub unconditional_assets: Option<Vec<String>>,
    pub limit: Option<u32>,
    pub wallet_address: Option<String>,
}

impl AssetsSearchParams {
    pub fn new(search_string: impl Into<String>) -> Self {
        Self {
            search_string: search_string.into(),
            condition: None,
            unconditional_assets: None,
            limit: None,
            wallet_address: None,
        }
    }
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

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct ReverseSwapSimulateParams {
    pub offer_address: String,
    pub ask_address: String,
    pub units: String,
    pub slippage_tolerance: String,
    pub pool_address: Option<String>,
    pub referral_address: Option<String>,
    pub referral_fee_bps: Option<String>,
    pub dex_v2: Option<bool>,
    pub dex_version: Option<Vec<String>>,
}

impl ReverseSwapSimulateParams {
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
            pool_address: None,
            referral_address: None,
            referral_fee_bps: None,
            dex_v2: Some(true),
            dex_version: None,
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
pub struct MarketsParams {
    pub dex_v2: bool,
}

impl Default for MarketsParams {
    fn default() -> Self { Self { dex_v2: true } }
}

impl MarketsParams {
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
pub struct FarmByPoolParams {
    pub pool_address: String,
}

impl FarmByPoolParams {
    pub fn new(pool_address: impl Into<String>) -> Self {
        Self {
            pool_address: pool_address.into(),
        }
    }
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
pub struct PoolQueryParams {
    pub condition: Option<String>,
    pub dex_v2: Option<bool>,
    pub limit: Option<u32>,
    pub search_terms: Vec<String>,
    pub sort_by: Vec<String>,
    pub unconditional_assets: Vec<String>,
    pub wallet_address: Option<String>,
}

impl PoolQueryParams {
    pub fn new() -> Self {
        Self {
            dex_v2: Some(true),
            ..Self::default()
        }
    }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct SwapStatusParams {
    pub router_address: String,
    pub owner_address: String,
    pub query_id: String,
}

impl SwapStatusParams {
    pub fn new(
        router_address: impl Into<String>,
        owner_address: impl Into<String>,
        query_id: impl Into<String>,
    ) -> Self {
        Self {
            router_address: router_address.into(),
            owner_address: owner_address.into(),
            query_id: query_id.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsDexParams {
    pub since: Option<String>,
    pub until: Option<String>,
}

impl StatsDexParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct StatsFeeAccrualsParams {
    pub referrer_address: String,
    pub since: String,
    pub until: String,
}

impl StatsFeeAccrualsParams {
    pub fn new(referrer_address: impl Into<String>, since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            referrer_address: referrer_address.into(),
            since: since.into(),
            until: until.into(),
        }
    }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct StatsFeeWithdrawalsParams {
    pub referrer_address: String,
    pub since: String,
    pub until: String,
}

impl StatsFeeWithdrawalsParams {
    pub fn new(referrer_address: impl Into<String>, since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            referrer_address: referrer_address.into(),
            since: since.into(),
            until: until.into(),
        }
    }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct StatsFeesParams {
    pub referrer_address: String,
    pub since: String,
    pub until: String,
}

impl StatsFeesParams {
    pub fn new(referrer_address: impl Into<String>, since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            referrer_address: referrer_address.into(),
            since: since.into(),
            until: until.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsOperationsParams {
    pub since: String,
    pub until: String,
    pub pool_address: Option<String>,
}

impl StatsOperationsParams {
    pub fn new(since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            since: since.into(),
            until: until.into(),
            pool_address: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsPoolParams {
    pub since: String,
    pub until: String,
    pub pool_address: Option<String>,
}

impl StatsPoolParams {
    pub fn new(since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            since: since.into(),
            until: until.into(),
            pool_address: None,
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
