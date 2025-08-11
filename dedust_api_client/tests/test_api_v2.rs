use anyhow::Result;
use dedust_api_client::api_v2::req::{RoutingPlanParams, V2Req};
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

#[tokio::test]
#[ignore = "TODO: Server response with 500: an internal server error occurred"]
async fn test_routing_plan() -> Result<()> {
    let client = init_env();
    let hmstr_addr = "0:09f2e59dec406ab26a5259a45d7ff23ef11f3e5c7c21de0b0d2a1cbe52b76b3d".to_string();
    let ton_addr = "0:0000000000000000000000000000000000000000000000000000000000000000".to_string();
    let params = RoutingPlanParams::new(&hmstr_addr, &ton_addr, &10_0000.to_string());
    let req = V2Req::RoutingPlan(params);
    let V2Rsp::RoutingPlan(rsp) = client.v2_exec(&req).await? else {
        panic!("V2Rsp::RoutingPlan expected")
    };
    assert!(!rsp.is_empty());
    Ok(())
}
