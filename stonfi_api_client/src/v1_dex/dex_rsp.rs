use crate::v1_dex::types::{Asset, Farm, Pool, QueryAsset, Router};
use serde_derive::Deserialize;
use crate::v1_dex::TransactionActionTree;

#[macro_export]
macro_rules! unwrap_rsp {
    ($variant:ident, $result:expr) => {
        match $result {
            V1DexRsp::$variant(inner) => Ok(inner),
            _ => Err(ApiClientsError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                $result
            ))),
        }
    };
}

#[derive(Deserialize, Debug, Clone)]
pub enum V1DexRsp {
    Assets(AssetsRsp),
    AssetsQuery(AssetsQueryRsp),
    Asset(AssetRsp),
    Farms(FarmsRsp),
    Farm(FarmRsp),
    Pools(PoolsRsp),
    Pool(PoolRsp),
    Routers(RoutersRsp),
    Router(RouterRsp),
    SwapSimulate(SwapSimulateRsp),
    TransactionActionTree(TransactionActionTreeRsp),
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetsRsp {
    pub asset_list: Vec<Asset>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetsQueryRsp {
    pub asset_list: Vec<QueryAsset>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetRsp {
    pub asset: Asset,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FarmsRsp {
    pub farm_list: Vec<Farm>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FarmRsp {
    pub farm: Farm,
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

pub type TransactionActionTreeRsp = TransactionActionTree;