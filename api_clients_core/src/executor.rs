use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{de, ser};
use crate::ApiClientError;
use crate::errors::ApiClientsResult;

pub struct Executor {
    api_url: String,
    http_cli: ClientWithMiddleware,
}

impl Executor {
    pub fn new(api_url: &str, retry_count: u32) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(retry_count);

        let http_cli = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self {
            api_url: api_url.to_string(),
            http_cli,
        }
    }

    pub async fn exec_get<RSP>(&self, path: &str) -> ApiClientsResult<RSP>
    where
        RSP: de::DeserializeOwned,
    {
        let empty_params = serde_json::Value::Null;
        self.exec_get_with_params(path, &empty_params).await
    }

    pub async fn exec_get_with_params<PARAMS, RSP>(&self, path: &str, params: &PARAMS) -> ApiClientsResult<RSP>
    where
        PARAMS: ser::Serialize,
        RSP: de::DeserializeOwned,
    {
        self.exec_impl(path, params, false).await
    }

    pub async fn exec_post<PARAMS, RSP>(&self, path: &str, params: &PARAMS) -> ApiClientsResult<RSP>
    where
        PARAMS: ser::Serialize,
        RSP: de::DeserializeOwned,
    {
        self.exec_impl(path, params, true).await
    }

    async fn exec_impl<PARAMS, RSP>(&self, path: &str, params: &PARAMS, is_post: bool) -> ApiClientsResult<RSP>
    where
        PARAMS: ser::Serialize,
        RSP: de::DeserializeOwned,
    {
        let get_params = serde_qs::to_string(params)?;
        let full_url = format!("{}/{path}?{get_params}", self.api_url);

        let req_type_str = if is_post { "POST" } else { "GET" };
        log::trace!("Executing {req_type_str} request to {full_url}");

        let req_builder = if is_post {
            self.http_cli.post(full_url)
        } else {
            self.http_cli.get(full_url)
        };

        let rsp = req_builder.send().await?;
        let rsp_code = rsp.status();
        let rsp_body = rsp.text().await?;

        match rsp_code.as_u16() {
            400..=499 => return Err(ApiClientError::ClientError(rsp_code.as_u16(), rsp_body)),
            500..=599 => return Err(ApiClientError::ServerError(rsp_code.as_u16(), rsp_body)),
            _ => {}
        }

        log::trace!("Got rsp_code: {rsp_code} rsp_body: '{rsp_body}'");
        Ok(serde_json::from_str(&rsp_body)?)
    }
}
