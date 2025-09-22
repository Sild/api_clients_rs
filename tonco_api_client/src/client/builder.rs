use crate::client::ToncoApiClient;
use crate::{GraphqlClient, DEFAULT_GRAPHQL_ENDPOINT};
use api_clients_core::Executor;
use std::sync::Arc;

#[derive(Default)]
pub struct ToncoApiClientBuilder {
    endpoint: Option<String>,
    retries: Option<u32>,
}

impl ToncoApiClientBuilder {
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    pub fn build(self) -> ToncoApiClient {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_GRAPHQL_ENDPOINT.to_owned());
        let retries = self.retries.unwrap_or(0);

        let executor = Arc::new(Executor::new(&endpoint, retries));

        let graphql = GraphqlClient { executor };

        ToncoApiClient { graphql }
    }
}
