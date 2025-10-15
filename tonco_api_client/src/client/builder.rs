use crate::ToncoApiClient;
use api_clients_core::Executor;
use derive_setters::Setters;
use std::sync::Arc;
pub static DEFAULT_GRAPHQL_ENDPOINT: &str = "https://indexer.tonco.io/";

#[derive(Setters)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    graphql_endpoint: String,
    retry_count: u32,
    executor: Option<Arc<Executor>>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            graphql_endpoint: DEFAULT_GRAPHQL_ENDPOINT.to_string(),
            retry_count: 3,
            executor: None,
        }
    }

    pub fn build(self) -> ToncoApiClient {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.graphql_endpoint).with_retry_count(self.retry_count).build().into(),
        };
        ToncoApiClient { executor }
    }
}
