use crate::client::{StonfiApiClient, DEFAULT_API_V1_URL};
use crate::v1_dex::V1DexClient;
use api_clients_core::{ApiClientsResult, Executor};
use derive_setters::Setters;
use std::sync::Arc;

#[derive(Setters)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    api_url: String,
    executor: Option<Arc<Executor>>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            api_url: DEFAULT_API_V1_URL.to_string(),
            executor: None,
        }
    }

    pub fn build(self) -> ApiClientsResult<StonfiApiClient> {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.api_url).build()?.into(),
        };

        let v1_dex = V1DexClient::new(executor);
        Ok(StonfiApiClient { v1_dex })
    }
}
