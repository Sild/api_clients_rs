use anyhow::Result;
use dedust_api_client::api_v2::req::{RoutingPlanParams, V2Req};
use dedust_api_client::api_v2::rsp::V2Rsp;
use dedust_api_client::client::{DedustApiClient, DedustApiClientConfig};
use dedust_api_client::unwrap_rsp;

fn init_env() -> DedustApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let config = DedustApiClientConfig {
        retry_count: 5,
        ..Default::default()
    };
    DedustApiClient::new_with_config(config)
}

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_env();
    let req = V2Req::Assets;
    let rsp = unwrap_rsp!(Assets, client.v2_exec(&req).await?)?;
    assert_ne!(rsp, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let req = V2Req::Pools;
    let rsp = unwrap_rsp!(Pools, client.v2_exec(&req).await?)?;
    assert_ne!(rsp, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools_lite() -> Result<()> {
    let client = init_env();
    let req = V2Req::PoolsLite;
    let rsp = unwrap_rsp!(PoolsLite, client.v2_exec(&req).await?)?;
    assert_ne!(rsp, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pool_trades() -> Result<()> {
    let client = init_env();
    let req = V2Req::PoolTrades("EQAADLqcF3lNb1O_GQLowwky8vvUGXuzPRNGvKBwBxjsHR7s".to_string());
    let rsp = unwrap_rsp!(PoolTrades, client.v2_exec(&req).await?)?;
    assert_ne!(rsp, vec![]);
    Ok(())
}

#[tokio::test]
#[ignore = "TODO: Server response with 500: an internal server error occurred"]
async fn test_routing_plan() -> Result<()> {
    let client = init_env();
    let usdt_addr = "0:b113a994b5024a16719f69139328eb759596c38a25f59028b146fecdc3621dfe";
    let ton_addr = "0:0000000000000000000000000000000000000000000000000000000000000000";
    let params = RoutingPlanParams::new(usdt_addr, ton_addr, &10.to_string());
    let req = V2Req::RoutingPlan(params);
    let rsp = unwrap_rsp!(RoutingPlan, client.v2_exec(&req).await?)?;
    assert!(!rsp.is_empty());
    Ok(())
}
