use crate::client::builder::ToncoApiClientBuilder;
use crate::GraphqlClient;

mod builder;

pub struct ToncoApiClient {
    pub graphql: GraphqlClient,
}

impl ToncoApiClient {
    pub fn builder() -> ToncoApiClientBuilder { ToncoApiClientBuilder::default() }
}
