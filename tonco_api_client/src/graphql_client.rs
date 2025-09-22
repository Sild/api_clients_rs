pub use graphql_req::*;
pub use graphql_rsp::*;
pub use graphql_types::*;

mod graphql_req;
mod graphql_rsp;
mod graphql_types;

use api_clients_core::{ApiClientsResult, Executor};
use graphql_client::{GraphQLQuery, Response};
use std::sync::Arc;

pub static DEFAULT_GRAPHQL_ENDPOINT: &str = "https://indexer.tonco.io/";

pub struct GraphqlClient {
    pub(super) executor: Arc<Executor>,
}

impl GraphqlClient {
    pub async fn exec(&self, req: &GraphqlReq) -> ApiClientsResult<GraphqlRsp> {
        let rsp = match req {
            GraphqlReq::Pools => {
                let query = Pools::build_query(pools::Variables);
                let headers = make_headers(pools::OPERATION_NAME);
                let result = self.executor.exec_post_body("", &query, &headers).await?;
                GraphqlRsp::Pools(handle_graphql_result(result)?)
            }
            GraphqlReq::Jettons => {
                let query = Jettons::build_query(jettons::Variables {});
                let headers = make_headers(jettons::OPERATION_NAME);
                let result = self.executor.exec_post_body("", &query, &headers).await?;
                GraphqlRsp::Jettons(handle_graphql_result(result)?)
            }
        };
        Ok(rsp)
    }
}

fn handle_graphql_result<R>(graphql_result: Response<R>) -> ApiClientsResult<R> {
    if let Some(errors) = graphql_result.errors {
        let msgs: Vec<String> = errors.into_iter().map(|e| e.message).collect();
        let err_msg = msgs.join(", ");
        Err(api_clients_core::ApiClientError::UnknownError(err_msg))
    } else if let Some(data) = graphql_result.data {
        Ok(data)
    } else {
        Err(api_clients_core::ApiClientError::UnexpectedResponse("No data in GraphQL response".to_string()))
    }
}

fn make_headers(op_name: &str) -> Vec<(String, String)> {
    vec![
        (reqwest::header::CONTENT_TYPE.to_string(), "application/json".to_string()),
        ("x-apollo-operation-name".to_string(), op_name.to_string()),
    ]
}
