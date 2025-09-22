// Use the following commands to update `schema.json`:
// cargo install graphql_client_cli --force
// graphql-client introspect-schema https://indexer.tonco.io > src/graphql/schema.json

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql_client/schema.json",
    query_path = "src/graphql_client/pools.graphql",
    response_derives = "Debug,Clone"
)]
pub struct Pools;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql_client/schema.json",
    query_path = "src/graphql_client/jettons.graphql",
    response_derives = "Debug,Clone"
)]
pub struct Jettons;
