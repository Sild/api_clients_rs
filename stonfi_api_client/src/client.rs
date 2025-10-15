mod builder;
use crate::client::builder::Builder;
use crate::v1_dex::V1DexClient;

pub struct StonfiApiClient {
    pub v1_dex: V1DexClient,
}

impl StonfiApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
