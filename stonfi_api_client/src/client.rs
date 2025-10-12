mod builder;
mod v1_dex;
use crate::client::builder::Builder;
use crate::client::v1_dex::V1Dex;

pub struct StonfiApiClient {
    pub v1_dex: V1Dex,
}

impl StonfiApiClient {
    pub fn builder() -> Builder { Builder::default() }
}
