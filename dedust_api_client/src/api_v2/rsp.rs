use crate::api_v2::types::{Pool, PoolLite, PoolTrade, RoutingPlanStep};
use std::collections::HashMap;

pub enum V2Rsp {
    Pools(Vec<Pool>),
    PoolsLite(Vec<PoolLite>),
    PoolTrades(Vec<PoolTrade>),
    RoutingPlan(HashMap<String, Vec<RoutingPlanStep>>),
}
