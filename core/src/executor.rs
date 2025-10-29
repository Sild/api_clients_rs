mod builder;

use crate::errors::ApiClientResult;
use crate::ApiClientsError;
use crate::ApiClientsError::*;
use reqwest::Response;
use reqwest_middleware::ClientWithMiddleware;
use serde::{de, ser};
use std::sync::Arc;

pub struct Executor {
    api_url: String,
    http_client: Arc<ClientWithMiddleware>,
}

impl Executor {
    pub fn builder<T: Into<String>>(api_endpoint: T) -> builder::Builder { builder::Builder::new(api_endpoint.into()) }

    pub async fn exec_get<RSP>(&self, path: &str) -> ApiClientResult<RSP>
    where
        RSP: de::DeserializeOwned,
    {
        self.exec_get_extra(path, &serde_json::Value::Null, &[]).await
    }

    pub async fn exec_get_extra<PARAMS, RSP>(
        &self,
        path: &str,
        params: &PARAMS,
        headers: &[(String, String)],
    ) -> ApiClientResult<RSP>
    where
        PARAMS: ser::Serialize,
        RSP: de::DeserializeOwned,
    {
        let get_params = serde_qs::to_string(params).map_err(|x| InvalidArgs(x.to_string()))?;
        let full_url = format!("{}/{path}?{get_params}", self.api_url);
        log::trace!("Executing GET request to {full_url}");

        let mut req_builder = self.http_client.get(full_url);
        for (key, value) in headers {
            req_builder = req_builder.header(key, value);
        }

        handle_response(req_builder.send().await?).await
    }

    // put params as query string
    pub async fn exec_post_qs<PARAMS, RSP>(
        &self,
        path: &str,
        params: &PARAMS,
        headers: &[(String, String)],
    ) -> ApiClientResult<RSP>
    where
        PARAMS: ser::Serialize,
        RSP: de::DeserializeOwned,
    {
        let get_params = serde_qs::to_string(params).map_err(|x| ApiClientsError::InvalidArgs(x.to_string()))?;
        let full_url = format!("{}/{path}?{get_params}", self.api_url);
        log::trace!("Executing POST request to {full_url}");

        let mut req_builder = self.http_client.post(full_url);
        for (key, value) in headers {
            req_builder = req_builder.header(key, value);
        }

        handle_response(req_builder.send().await?).await
    }

    // put params as body in json format
    pub async fn exec_post_body<PARAMS, RSP>(
        &self,
        path: &str,
        params: &PARAMS,
        headers: &[(String, String)],
    ) -> ApiClientResult<RSP>
    where
        PARAMS: ser::Serialize,
        RSP: de::DeserializeOwned,
    {
        let full_url = format!("{}/{path}", self.api_url);
        log::trace!("Executing POST request to {full_url}");

        let mut req_builder = self.http_client.post(full_url);
        for (key, value) in headers {
            req_builder = req_builder.header(key, value);
        }
        let body = serde_json::to_string(params).map_err(|x| InvalidArgs(x.to_string()))?;
        req_builder = req_builder.body(body);

        handle_response(req_builder.send().await?).await
    }
}

async fn handle_response<RSP>(response: Response) -> ApiClientResult<RSP>
where
    RSP: de::DeserializeOwned,
{
    let rsp_code = response.status();
    let rsp_body = response.text().await?;

    match rsp_code.as_u16() {
        400..=499 => return Err(ClientError(rsp_code.as_u16(), rsp_body)),
        500..=599 => return Err(ServerError(rsp_code.as_u16(), rsp_body)),
        _ => {}
    }

    log::trace!("Got rsp_code: {rsp_code} rsp_body: '{rsp_body}'");

    match serde_json::from_str(&rsp_body) {
        Ok(rsp) => Ok(rsp),
        Err(err) => {
            log::warn!("Failed to parse response body: '{rsp_body}', err: {err}");
            Err(UnexpectedResponse(err.to_string()))
        }
    }
}
