const API_V1_URL: &str = "https://api.ston.fi/v1";

pub struct StonfiApiClientConfig {
    pub api_url: String,
    pub retry_count: u32,
}

impl Default for StonfiApiClientConfig {
    fn default() -> Self {
        Self {
            api_url: API_V1_URL.to_string(),
            retry_count: 3,
        }
    }
}
