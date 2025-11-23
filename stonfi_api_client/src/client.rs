mod builder;

use crate::client::builder::Builder;
use crate::v1::V1Client;
pub const DEFAULT_API_V1_URL: &str = "https://api.ston.fi/v1";

pub struct StonfiApiClient {
    pub v1: V1Client,
}

impl StonfiApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
