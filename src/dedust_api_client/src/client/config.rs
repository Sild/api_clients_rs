const API_V2_URL: &str = "https://api.dedust.io/v2";

pub struct DedustApiClientConfig {
    pub api_url: String,
    pub retry_count: u32,
}

impl Default for DedustApiClientConfig {
    fn default() -> Self {
        Self {
            api_url: API_V2_URL.to_string(),
            retry_count: 3,
        }
    }
}
