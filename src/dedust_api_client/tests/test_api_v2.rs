use anyhow::Result;
use dedust_api_client::api_v2::req::V2Req;
use dedust_api_client::api_v2::rsp::V2Rsp;
use dedust_api_client::client::{DedustApiClient, DedustApiClientConfig};

fn init_env() -> DedustApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let config = DedustApiClientConfig {
        retry_count: 5,
        ..Default::default()
    };
    DedustApiClient::new_with_config(config)
}

#[tokio::test]
async fn test_get_pools() -> Result<()> {
    let client = init_env();
    let req = V2Req::Pools;
    let V2Rsp::Pools(rsp) = client.v2_exec(&req).await? else {
        panic!("V2Rsp::Pools expected")
    };
    assert_ne!(rsp, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_get_pools_lite() -> Result<()> {
    let client = init_env();
    let req = V2Req::PoolsLite;
    let V2Rsp::PoolsLite(rsp) = client.v2_exec(&req).await? else {
        panic!("V2Rsp::PoolsLite expected")
    };
    assert_ne!(rsp, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_get_pool_trades() -> Result<()> {
    let client = init_env();
    let req = V2Req::PoolTrades("EQAADLqcF3lNb1O_GQLowwky8vvUGXuzPRNGvKBwBxjsHR7s".to_string());
    let V2Rsp::PoolTrades(rsp) = client.v2_exec(&req).await? else {
        panic!("V2Rsp::PoolTrades expected")
    };
    assert_ne!(rsp, vec![]);
    Ok(())
}
