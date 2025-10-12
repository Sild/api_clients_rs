mod builder;

use crate::client::builder::Builder;
use api_clients_core::{ApiClientError, ApiClientResult, Executor};
use graphql_client::Response;
use serde::{de, ser};
use std::sync::Arc;

pub struct ToncoApiClient {
    executor: Arc<Executor>,
}

impl ToncoApiClient {
    pub fn builder() -> Builder { Default::default() }

    pub async fn exec_graphql<PARAMS, RSP>(&self, op_name: &str, graphql_query: &PARAMS) -> ApiClientResult<RSP>
    where
        PARAMS: ser::Serialize,
        RSP: de::DeserializeOwned,
    {
        let headers = &[
            (reqwest::header::CONTENT_TYPE.to_string(), "application/json".to_string()),
            ("x-apollo-operation-name".to_string(), op_name.to_string()),
        ];
        let rsp = self.executor.exec_post_body("", graphql_query, headers).await?;
        handle_graphql_result(rsp)
    }
}

fn handle_graphql_result<R>(graphql_result: Response<R>) -> ApiClientResult<R> {
    if let Some(errors) = graphql_result.errors {
        let msgs: Vec<String> = errors.into_iter().map(|e| e.message).collect();
        let err_msg = msgs.join(", ");
        Err(ApiClientError::UnexpectedResponse(err_msg))
    } else if let Some(data) = graphql_result.data {
        Ok(data)
    } else {
        Err(ApiClientError::UnknownError("No data in GraphQL response".to_string()))
    }
}
