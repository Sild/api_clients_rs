use crate::{jettons, pools};

#[macro_export]
macro_rules! unwrap_rsp {
    ($variant:ident, $result:expr) => {
        match $result {
            GraphqlRsp::$variant(inner) => Ok(inner),
            _ => Err(ApiClientError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                $result
            ))),
        }
    };
}

#[derive(Debug, Clone)]
pub enum GraphqlRsp {
    Pools(pools::ResponseData),
    Jettons(jettons::ResponseData),
}
