use std::collections::HashMap;
use crate::api_v2::types::{Pool, PoolLite, PoolTrade, RoutingPlanStep};

pub enum V2Rsp {
    Pools(Vec<Pool>),
    PoolsLite(Vec<PoolLite>),
    PoolTrades(Vec<PoolTrade>),
    RoutingPlan(HashMap<String, Vec<RoutingPlanStep>>),
}
