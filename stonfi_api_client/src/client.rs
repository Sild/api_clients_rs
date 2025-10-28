mod builder;
pub use builder::*;

use crate::v1_dex::V1DexClient;
pub const DEFAULT_API_V1_URL: &str = "https://api.ston.fi/v1";

pub struct StonfiApiClient {
    pub v1_dex: V1DexClient,
}

impl StonfiApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
