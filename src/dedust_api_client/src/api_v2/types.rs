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
pub struct Pool {
    pub address: String,
    pub lt: String,
    #[serde(rename = "totalSupply")]
    pub total_supply: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    #[serde(rename = "tradeFee")]
    pub trade_fee: String,
    pub assets: Vec<Asset>,
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    pub reserves: Vec<String>,
    pub stats: Stats,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PoolLite {
    pub address: String,
    pub lt: String,
    #[serde(rename = "totalSupply")]
    pub total_supply: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    #[serde(rename = "tradeFee")]
    pub trade_fee: String,
    pub assets: Vec<String>,
    pub reserves: Vec<String>,
    pub fees: Vec<String>,
    pub volume: Vec<String>,
}
