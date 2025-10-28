use crate::api_v2::types::*;
use serde_derive::Deserialize;

#[macro_export]
macro_rules! unwrap_rsp {
    ($variant:ident, $result:expr) => {
        match $result {
            V2Rsp::$variant(inner) => Ok(inner),
            _ => Err(ApiClientsError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                $result
            ))),
        }
    };
}

#[derive(Deserialize, Debug, Clone)]
pub enum V2Rsp {
    Assets(Vec<Asset>),
    Pools(Vec<Pool>),
    PoolsLite(Vec<PoolLite>),
    PoolTrades(Vec<PoolTrade>),
    RoutingPlan(Vec<Vec<RoutingPlanStep>>),
}
