mod builder;

use crate::api_client::builder::Builder;
use crate::v1::V1ApiClient;
pub const DEFAULT_API_V1_URL: &str = "https://api.ston.fi/v1";

#[derive(Clone)]
pub struct StonfiApiClient {
    pub v1: V1ApiClient,
}

impl StonfiApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
