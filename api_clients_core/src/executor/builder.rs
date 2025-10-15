use crate::Executor;
use derive_setters::Setters;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;

#[derive(Setters, Debug)]
#[setters(prefix = "with_", strip_option)]
pub struct Builder {
    #[setters(skip)]
    api_url: String,
    retry_count: u32,
}

impl Builder {
    pub(super) fn new(api_url: String) -> Self {
        Self {
            api_url,
            retry_count: 3,
        }
    }

    pub fn build(self) -> Executor {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(self.retry_count);

        let http_cli = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Executor {
            api_url: self.api_url,
            http_cli,
        }
    }
}
