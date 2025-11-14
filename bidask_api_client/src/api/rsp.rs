use serde::Deserialize;

use crate::api::types::*;

#[macro_export]
macro_rules! unwrap_rsp {
    ($variant:ident, $result:expr) => {
        match $result {
            Rsp::$variant(inner) => Ok(inner),
            _ => Err(ApiClientsError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                $result
            ))),
        }
    };
}

#[derive(Deserialize, Debug, Clone)]
pub enum Rsp {
    Pools(Vec<PoolInfo>),
}
