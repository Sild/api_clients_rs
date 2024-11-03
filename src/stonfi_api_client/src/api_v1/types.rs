use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Meta {
    custom_payload_api_uri: String,
    decimals: i32,
    display_name: String,
    image_url: String,
    symbol: String,
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
