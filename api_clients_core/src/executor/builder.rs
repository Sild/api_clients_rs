use crate::Executor;
use derive_setters::Setters;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::sync::Arc;

#[derive(Setters, Debug)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    #[setters(skip)]
    api_url: String,
    retry_count: u32,
    http_client: Option<Arc<ClientWithMiddleware>>,
}

impl Builder {
    pub(super) fn new(api_url: String) -> Self {
        Self {
            api_url,
            retry_count: 3,
            http_client: None,
        }
    }

    pub fn build(self) -> Executor {
        let http_client = match self.http_client {
            Some(cli) => cli,
            None => {
                let retry = ExponentialBackoff::builder().build_with_max_retries(self.retry_count);
                ClientBuilder::new(reqwest::Client::new())
                    .with(RetryTransientMiddleware::new_with_policy(retry))
                    .build()
                    .into()
            }
        };

        Executor {
            api_url: self.api_url,
            http_client,
        }
    }
}
