use crate::v1::types::{Asset, Farm, Pool, QueryAsset, Router};
use crate::v1::{TransactionActionTree, TxId};
use derive_more::From;
use serde_derive::Deserialize;

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
    Asset(AssetResponse),
    Farms(FarmsResponse),
    Farm(FarmResponse),
    Pools(PoolsResponse),
    Pool(PoolResponse),
    Routers(RoutersResponse),
    Router(RouterResponse),
    SwapSimulate(SwapSimulateResponse),
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
#[non_exhaustive]
pub struct TransactionQueryResponse {
    pub tx_id: Option<TxId>,
    pub wallet_seqno: Option<u32>,
}

pub type TransactionActionTreeResponse = TransactionActionTree;
