use crate::client::{StonfiApiClient, DEFAULT_API_V1_URL};
use crate::v1::V1Client;
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

        let v1_client = V1Client::new(executor);
        Ok(StonfiApiClient { v1: v1_client })
    }
}
