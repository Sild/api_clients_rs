use crate::{ApiClientsError, ApiClientsResult, Executor};
use derive_setters::Setters;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::sync::Arc;
use std::time::Duration;

#[derive(Setters, Debug)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    #[setters(skip)]
    api_url: String,
    retry_count: u32,
    timeout: Duration,
    http_client: Option<Arc<ClientWithMiddleware>>,
}

impl Builder {
    pub(super) fn new(api_url: String) -> Self {
        Self {
            api_url,
            retry_count: 3,
            timeout: Duration::from_secs(10),
            http_client: None,
        }
    }

    pub fn build(self) -> ApiClientsResult<Executor> {
        let http_client = match self.http_client {
            Some(cli) => cli,
            None => {
                let retry = ExponentialBackoff::builder().build_with_max_retries(self.retry_count);
                let client = reqwest::ClientBuilder::new()
                    .timeout(self.timeout)
                    .build()
                    .map_err(|err| ApiClientsError::Internal(err.to_string()))?;

                ClientBuilder::new(client).with(RetryTransientMiddleware::new_with_policy(retry)).build().into()
            }
        };

        let executor = Executor {
            api_endpoint: self.api_url,
            http_client,
        };
        Ok(executor)
    }
}
