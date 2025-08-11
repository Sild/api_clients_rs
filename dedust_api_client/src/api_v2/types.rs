use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Stats {
    pub fees: Vec<String>,
    pub volume: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub image: String,
    pub decimals: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Asset {
    #[serde(rename = "type")]
    pub asset_type: String,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    pub address: String,
    pub lt: String,
    pub total_supply: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub trade_fee: String,
    pub assets: Vec<Asset>,
    pub last_price: Option<String>,
    pub reserves: Vec<String>,
    pub stats: Stats,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PoolLite {
    pub address: String,
    pub lt: String,
    pub total_supply: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub trade_fee: String,
    pub assets: Vec<String>,
    pub reserves: Vec<String>,
    pub fees: Vec<String>,
    pub volume: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PoolAsset {
    #[serde(rename = "type")]
    pub asset_type: String,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PoolTrade {
    pub sender: String,
    pub asset_in: PoolAsset,
    pub asset_out: PoolAsset,
    pub amount_in: String,
    pub amount_out: String,
    pub lt: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PoolRoutingPlan {
    pub address: String,
    pub is_stable: bool,
    pub assets: Vec<String>,
    pub reserves: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoutingPlanStep {
    pub pool: PoolRoutingPlan,
    pub asset_in: String,
    pub asset_out: String,
    pub trade_fee: String,
    pub amount_in: String,
    pub amount_out: String,
}
