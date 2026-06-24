use anyhow::Result;
use dedust_api_client::api_client::DedustApiClient;
use dedust_api_client::api_v2::{RoutingPlanParams, V2Request};
use dedust_api_client::unwrap_response;

fn init_env() -> DedustApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    DedustApiClient::builder().build().unwrap()
}

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_env();
    let request = V2Request::Assets;
    let response = unwrap_response!(Assets, client.exec_api_v2(&request).await?)?;
    assert_ne!(response, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let request = V2Request::Pools;
    let response = unwrap_response!(Pools, client.exec_api_v2(&request).await?)?;
    assert_ne!(response, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools_lite() -> Result<()> {
    let client = init_env();
    let request = V2Request::PoolsLite;
    let response = unwrap_response!(PoolsLite, client.exec_api_v2(&request).await?)?;
    assert_ne!(response, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pool_trades() -> Result<()> {
    let client = init_env();
    let request = V2Request::PoolTrades("EQAADLqcF3lNb1O_GQLowwky8vvUGXuzPRNGvKBwBxjsHR7s".to_string());
    let response = unwrap_response!(PoolTrades, client.exec_api_v2(&request).await?)?;
    assert_ne!(response, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_routing_plan() -> Result<()> {
    let client = init_env();
    let usdt_addr = "0:b113a994b5024a16719f69139328eb759596c38a25f59028b146fecdc3621dfe";
    let ton_addr = "0:0000000000000000000000000000000000000000000000000000000000000000";
    let params = RoutingPlanParams::new(usdt_addr, ton_addr, &10.to_string());
    let response = unwrap_response!(RoutingPlan, client.exec_api_v2(params).await?)?;
    assert_ne!(response.len(), 0);
    assert_ne!(response[0], vec![]);
    Ok(())
}
