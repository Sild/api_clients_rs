use crate::api_v1::types::*;
use serde_derive::Deserialize;

#[macro_export]
macro_rules! unwrap_rsp {
    ($variant:ident, $result:expr) => {
        match $result {
            V1Rsp::$variant(inner) => Ok(inner),
            _ => Err(ApiClientError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                $result
            ))),
        }
    };
}

#[derive(Deserialize, Debug, Clone)]
pub enum V1Rsp {
    Assets(Vec<Token>),
    Pools(Vec<PoolsRsp>),
}

#[derive(Deserialize, Debug, Clone)]
pub struct PoolsRsp {
    pub total_count: i64,
    pub pools: Vec<PoolInfo>,
}
