use crate::v1::types::{Asset, Farm, Pool, QueryAsset, Router};
use crate::v1::{TransactionActionTree, TxId};
use derive_more::From;
use serde_derive::Deserialize;
use std::collections::BTreeMap;

#[macro_export]
macro_rules! unwrap_response {
    ($variant:ident, $result:expr) => {
        match $result {
            $crate::v1::V1Response::$variant(inner) => Ok(inner),
            other => Err($crate::api_clients_core::ApiClientsError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                other
            ))),
        }
    };
}

#[derive(Deserialize, Debug, Clone, From)]
#[non_exhaustive]
pub enum V1Response {
    Assets(AssetsResponse),
    AssetsQuery(AssetsQueryResponse),
    AssetsSearch(AssetsSearchResponse),
    Asset(AssetResponse),
    Farms(FarmsResponse),
    Farm(FarmResponse),
    #[from(skip)]
    FarmByPool(FarmsResponse),
    Markets(MarketsResponse),
    Pools(PoolsResponse),
    #[from(skip)]
    PoolQuery(PoolsResponse),
    Pool(PoolResponse),
    Routers(RoutersResponse),
    Router(RouterResponse),
    SwapSimulate(SwapSimulateResponse),
    #[from(skip)]
    ReverseSwapSimulate(SwapSimulateResponse),
    SwapStatus(SwapStatusResponse),
    StatsDex(StatsDexResponse),
    StatsFeeAccruals(StatsFeeAccrualsResponse),
    StatsFeeWithdrawals(StatsFeeWithdrawalsResponse),
    StatsFees(StatsFeesResponse),
    StatsOperations(StatsOperationsResponse),
    StatsPool(StatsPoolResponse),
    StatsStaking(StatsStakingResponse),
    TransactionActionTree(TransactionActionTreeResponse),
    TransactionQuery(TransactionQueryResponse),
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct AssetsResponse {
    pub asset_list: Vec<Asset>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct AssetsQueryResponse {
    pub asset_list: Vec<QueryAsset>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct AssetsSearchResponse {
    pub asset_list: Vec<QueryAsset>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct AssetResponse {
    pub asset: Asset,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct FarmsResponse {
    pub farm_list: Vec<Farm>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct FarmResponse {
    pub farm: Farm,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct MarketsResponse {
    pub pairs: Vec<[String; 2]>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct PoolsResponse {
    pub pool_list: Vec<Pool>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct PoolResponse {
    pub pool: Pool,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct RoutersResponse {
    pub router_list: Vec<Router>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct RouterResponse {
    pub router: Router,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct SwapSimulateResponse {
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

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "@type")]
#[non_exhaustive]
pub enum SwapStatusResponse {
    Found(SwapStatus),
    NotFound,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct SwapStatus {
    pub address: String,
    pub balance_deltas: SwapStatusBalanceDeltas,
    pub coins: String,
    pub exit_code: String,
    pub logical_time: String,
    pub query_id: String,
    pub tx_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SwapStatusBalanceDeltas {
    Map(BTreeMap<String, String>),
    Text(String),
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsDexResponse {
    pub since: String,
    pub until: String,
    pub stats: DexStats,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct DexStats {
    pub trades: u64,
    pub tvl: String,
    pub unique_wallets: u64,
    pub volume_usd: String,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsFeeAccrualsResponse {
    pub operations: Vec<FeeAccrual>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct FeeAccrual {
    pub pool_address: String,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsFeeWithdrawalsResponse {
    pub withdrawals: Vec<FeeWithdrawal>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct FeeWithdrawal {
    pub vault_address: String,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsFeesResponse {
    pub assets_fee_stats: Vec<AssetFeeStats>,
    pub since: String,
    pub until: String,
    pub total_accrued_usd: Option<String>,
    pub total_withdrawn_usd: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct AssetFeeStats {
    pub accrued: String,
    pub accrued_usd: Option<String>,
    pub asset_address: String,
    pub withdrawn: String,
    pub withdrawn_usd: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsOperationsResponse {
    pub operations: Vec<StatsOperationInfo>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsOperationInfo {
    pub operation: StatsOperation,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsOperation {
    pub pool_address: String,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsPoolResponse {
    pub since: String,
    pub until: String,
    pub unique_wallets_count: u64,
    pub stats: Vec<PoolStats>,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct PoolStats {
    pub pool_address: String,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct StatsStakingResponse {
    pub gemston_total_supply: String,
    pub ston_price_usd: String,
    pub ston_total_supply: String,
    pub total_staked_ston: String,
}

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct TransactionQueryResponse {
    pub tx_id: Option<TxId>,
    pub wallet_seqno: Option<u32>,
}

pub type TransactionActionTreeResponse = TransactionActionTree;
