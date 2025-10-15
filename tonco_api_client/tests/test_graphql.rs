use graphql_client::GraphQLQuery;
use tonco_api_client::client::ToncoApiClient;

fn init_env() -> ToncoApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    ToncoApiClient::builder().build()
}

// Use the following commands to update `graphql_schema.json`:
// cargo install graphql_client_cli --force
// graphql-client introspect-schema https://indexer.tonco.io > schema.json
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql_schema.json",
    query_path = "tests/pools.graphql",
    response_derives = "Debug,Clone"
)]
pub struct Pools;

// example of using graphql_client to generate query
#[tokio::test]
async fn test_tonco_pools() -> anyhow::Result<()> {
    let client = init_env();
    let query = Pools::build_query(pools::Variables);
    let rsp: pools::ResponseData = client.exec_graphql(pools::OPERATION_NAME, &query).await?;
    assert!(rsp.pools.is_some());
    let pools = rsp.pools.unwrap();
    assert!(pools.len() > 0);
    assert!(pools[0].is_some());
    Ok(())
}
