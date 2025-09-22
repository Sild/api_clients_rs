use api_clients_core::ApiClientError;
use tonco_api_client::*;

fn init_env() -> ToncoApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();

    ToncoApiClient::builder().with_retries(5).build()
}

#[tokio::test]
async fn test_pools() -> anyhow::Result<()> {
    let client = init_env();
    let req = GraphqlReq::Pools;
    let rsp = unwrap_rsp!(Pools, client.graphql.exec(&req).await?)?;
    assert!(rsp.pools.is_some());
    assert!(rsp.pools.unwrap().len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_jettons() -> anyhow::Result<()> {
    let client = init_env();
    let req = GraphqlReq::Jettons;
    let rsp = unwrap_rsp!(Jettons, client.graphql.exec(&req).await?)?;
    assert!(rsp.jettons.is_some());
    assert!(rsp.jettons.unwrap().len() > 0);
    Ok(())
}
