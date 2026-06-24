mod builder;

use crate::errors::ApiClientsResult;
use crate::executor::builder::Builder;
use crate::ApiClientsError;
use crate::ApiClientsError::*;
use reqwest::Response;
use reqwest_middleware::ClientWithMiddleware;
use serde::{de, ser};
use std::sync::Arc;

#[non_exhaustive]
pub struct Executor {
    api_endpoint: String,
    http_client: Arc<ClientWithMiddleware>,
}

impl Executor {
    pub fn builder<T: Into<String>>(api_endpoint: T) -> Builder { Builder::new(api_endpoint.into()) }

    pub async fn exec_get<RESPONSE>(&self, path: &str) -> ApiClientsResult<RESPONSE>
    where
        RESPONSE: de::DeserializeOwned,
    {
        self.exec_get_extra(path, &serde_json::Value::Null, &[]).await
    }

    pub async fn exec_get_extra<PARAMS, RESPONSE>(
        &self,
        path: &str,
        params: &PARAMS,
        headers: &[(String, String)],
    ) -> ApiClientsResult<RESPONSE>
    where
        PARAMS: ser::Serialize,
        RESPONSE: de::DeserializeOwned,
    {
        let get_params = serde_qs::to_string(params).map_err(|x| InvalidArgs(x.to_string()))?;
        let full_url = format!("{}/{path}?{get_params}", self.api_endpoint);
        log::trace!("Executing GET request to {full_url}");

        let mut request_builder = self.http_client.get(full_url);
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }

        handle_response(request_builder.send().await?).await
    }

    // put params as query string
    pub async fn exec_post_qs<PARAMS, RESPONSE>(
        &self,
        path: &str,
        params: &PARAMS,
        headers: &[(String, String)],
    ) -> ApiClientsResult<RESPONSE>
    where
        PARAMS: ser::Serialize,
        RESPONSE: de::DeserializeOwned,
    {
        let get_params = serde_qs::to_string(params).map_err(|x| ApiClientsError::InvalidArgs(x.to_string()))?;
        let full_url = format!("{}/{path}?{get_params}", self.api_endpoint);
        log::trace!("Executing POST request to {full_url}");

        let mut request_builder = self.http_client.post(full_url);
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }

        handle_response(request_builder.send().await?).await
    }

    // put params as body in json format
    pub async fn exec_post_body<PARAMS, RESPONSE>(
        &self,
        path: &str,
        params: &PARAMS,
        headers: &[(String, String)],
    ) -> ApiClientsResult<RESPONSE>
    where
        PARAMS: ser::Serialize,
        RESPONSE: de::DeserializeOwned,
    {
        let full_url = format!("{}/{path}", self.api_endpoint);
        log::trace!("Executing POST request to {full_url}");

        let mut request_builder = self.http_client.post(full_url);
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }
        let body = serde_json::to_string(params).map_err(|x| InvalidArgs(x.to_string()))?;
        request_builder = request_builder.body(body);

        handle_response(request_builder.send().await?).await
    }
}

async fn handle_response<RESPONSE>(response: Response) -> ApiClientsResult<RESPONSE>
where
    RESPONSE: de::DeserializeOwned,
{
    let response_code = response.status();
    let response_body = response.text().await?;

    match response_code.as_u16() {
        400..=499 => return Err(Client(response_code.as_u16(), response_body)),
        500..=599 => return Err(Server(response_code.as_u16(), response_body)),
        _ => {}
    }

    log::trace!("Got response_code: {response_code} response_body: '{response_body}'");

    match serde_json::from_str(&response_body) {
        Ok(response) => Ok(response),
        Err(err) => {
            log::warn!("Failed to parse response body: '{response_body}', err: {err}");
            Err(UnexpectedResponse(err.to_string()))
        }
    }
}
