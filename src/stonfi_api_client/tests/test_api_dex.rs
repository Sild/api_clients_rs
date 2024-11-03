use anyhow::Result;
use stonfi_api_client::api_v1::dex_req_params::SwapSimulateParams;
use stonfi_api_client::client::{StonfiApiClient, StonfiApiClientConfig};

fn init_env() -> StonfiApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let config = StonfiApiClientConfig {
        retry_count: 5,
        ..Default::default()
    };
    StonfiApiClient::new_with_config(config)
}

#[tokio::test]
async fn test_get_assets() -> Result<()> {
    let client = init_env();
    let rsp = client.dex.get_assets().await?;
    assert_ne!(rsp.asset_list, vec![]);

    Ok(())
}

#[tokio::test]
async fn test_get_asset() -> Result<()> {
    let client = init_env();
    let ton_addr = "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c";
    let rsp = client.dex.get_asset(ton_addr).await?;
    assert!(rsp.is_some());
    let rsp = rsp.unwrap();
    assert_eq!(rsp.asset.contract_address, ton_addr);
    assert_eq!(rsp.asset.display_name, "TON");
    Ok(())
}

#[tokio::test]
async fn test_get_pools() -> Result<()> {
    let client = init_env();
    let rsp = client.dex.get_pools(false).await?;
    assert_ne!(rsp.pool_list, vec![]);

    Ok(())
}

#[tokio::test]
async fn test_get_pool() -> Result<()> {
    let client = init_env();
    let hmstr_usdt_pool = "EQBXg9I5MBvwv7O8Xd0ZOC6z7T6yoCojaBXQXoAYx6paDO2s";
    let rsp = client.dex.get_pool(hmstr_usdt_pool).await?;
    assert!(rsp.is_some());
    let rsp = rsp.unwrap();
    assert_eq!(rsp.pool.address, hmstr_usdt_pool);
    assert_eq!(rsp.pool.token0_address, "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo"); // hmstr
    assert_eq!(rsp.pool.token1_address, "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs"); // usdt
    Ok(())
}

#[tokio::test]
async fn test_get_routers() -> Result<()> {
    let client = init_env();
    let rsp = client.dex.get_routers(false).await?;
    assert_ne!(rsp.router_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_get_router() -> Result<()> {
    let client = init_env();
    let addr = "EQB3ncyBUTjZUA5EnFKR5_EnOMI9V1tTEAAPaiU71gc4TiUt";
    let rsp = client.dex.get_router(addr).await?;
    assert!(rsp.is_some());
    let rsp = rsp.unwrap();
    assert_eq!(rsp.router.address, addr);
    Ok(())
}

#[tokio::test]
async fn test_swap_simulate() -> Result<()> {
    let client = init_env();

    let req = SwapSimulateParams {
        offer_address: "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo".to_string(), // hmstr
        ask_address: "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs".to_string(),   // usdt
        units: "40000000".to_string(),
        slippage_tolerance: "0.001".to_string(),
        referral_address: None,
        referral_fee: None,
        dex_v2: Some(true),
    };

    let rsp = client.dex.swap_simulate(&req).await?;
    assert_eq!(rsp.offer_address, req.offer_address);
    assert_eq!(rsp.ask_address, req.ask_address);
    Ok(())
}
