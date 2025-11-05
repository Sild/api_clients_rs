use crate::client::{ToncoApiClient, DEFAULT_GRAPHQL_ENDPOINT};
use api_clients_core::{ApiClientsResult, Executor};
use derive_setters::Setters;
use std::sync::Arc;

#[derive(Setters)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    graphql_endpoint: String,
    executor: Option<Arc<Executor>>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            graphql_endpoint: DEFAULT_GRAPHQL_ENDPOINT.to_string(),
            executor: None,
        }
    }

    pub fn build(self) -> ApiClientsResult<ToncoApiClient> {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.graphql_endpoint).build()?.into(),
        };
        Ok(ToncoApiClient { executor })
    }
}
