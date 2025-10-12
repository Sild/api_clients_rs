use crate::ToncoApiClient;
use api_clients_core::Executor;
use derive_setters::Setters;
use std::sync::Arc;
pub static DEFAULT_GRAPHQL_ENDPOINT: &str = "https://indexer.tonco.io/";

#[derive(Setters, Debug, Default)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    graphql_endpoint: Option<String>,
    retry_count: Option<u32>,
}

impl Builder {
    pub fn build(self) -> ToncoApiClient {
        let endpoint = self.graphql_endpoint.unwrap_or_else(|| DEFAULT_GRAPHQL_ENDPOINT.to_string());
        let retry_count = self.retry_count.unwrap_or(3);
        let executor = Arc::new(Executor::new(&endpoint, retry_count));
        ToncoApiClient { executor }
    }
}
