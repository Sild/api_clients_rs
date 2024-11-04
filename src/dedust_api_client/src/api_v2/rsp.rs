use crate::api_v2::types::{Pool, PoolLite, PoolTrade};

pub enum V2Rsp {
    Pools(Vec<Pool>),
    PoolsLite(Vec<PoolLite>),
    PoolTrades(Vec<PoolTrade>),
}
