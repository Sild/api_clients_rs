use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Meta {
    pub custom_payload_api_uri: Option<String>,
    pub decimals: Option<i32>,
    pub display_name: Option<String>,
    pub image_url: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Asset {
    pub contract_address: String,
    pub symbol: String,
    pub display_name: String,
    pub decimals: u32,
    pub kind: String,
    pub deprecated: bool,
    pub community: bool,
    pub blacklisted: bool,
    pub default_symbol: bool,
    pub taxable: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct QueryAsset {
    pub balance: Option<String>,
    pub contract_address: String,
    pub dex_price_usd: Option<String>,
    pub extensions: Option<Vec<String>>,
    pub kind: String,
    pub meta: Option<Meta>,
    pub pair_priority: Option<i64>,
    pub popularity_index: Option<f64>,
    pub tags: Vec<String>,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Pool {
    pub address: String,
    pub collected_token0_protocol_fee: String,
    pub collected_token1_protocol_fee: String,
    pub deprecated: bool,
    pub lp_fee: String,
    pub lp_price_usd: String,
    pub lp_total_supply: String,
    pub lp_total_supply_usd: String,
    pub protocol_fee: String,
    pub protocol_fee_address: String,
    pub reserve0: String,
    pub reserve1: String,
    pub router_address: String,
    pub token0_address: String,
    pub token1_address: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Router {
    pub address: String,
    pub major_version: i64,
    pub minor_version: i64,
    pub pool_creation_enabled: bool,
    pub pton_master_address: String,
    pub pton_version: String,
    pub pton_wallet_address: String,
    pub router_type: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FarmReward {
    pub address: Option<String>,
    pub remaining_rewards: String,
    pub reward_rate_24h: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FarmNftReward {
    pub address: String,
    pub amount: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FarmNft {
    pub address: String,
    pub create_timestamp: String,
    pub min_unstake_timestamp: String,
    pub nonclaimed_rewards: String,
    pub rewards: Vec<FarmNftReward>,
    pub staked_tokens: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Farm {
    pub locked_total_lp: String,
    pub locked_total_lp_usd: String,
    pub min_stake_duration_s: String,
    pub minter_address: String,
    pub nft_infos: Vec<FarmNft>,
    pub pool_address: String,
    pub rewards: Vec<FarmReward>,
    pub status: String,
}
