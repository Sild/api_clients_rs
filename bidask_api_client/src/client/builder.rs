use crate::client::{BidaskApiClient, DEFAULT_API_URL};
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
            api_url: DEFAULT_API_URL.to_string(),
            executor: None,
        }
    }

    pub fn build(self) -> ApiClientsResult<BidaskApiClient> {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.api_url).build()?.into(),
        };

        Ok(BidaskApiClient { executor })
    }
}
