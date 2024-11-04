use anyhow::Result;
use stonfi_api_client::api_v1::dex_req::{PoolsParams, RoutersParams, SwapSimulateParams, V1DexReq};
use stonfi_api_client::api_v1::dex_rsp::V1DexRsp;
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
    let req = V1DexReq::Assets;
    let V1DexRsp::Assets(rsp) = client.dex.v1_exec(&req).await? else { panic!("AssetsRsp expected") };
    assert_ne!(rsp.asset_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_get_asset() -> Result<()> {
    let client = init_env();
    let ton_addr = "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c";
    let req = V1DexReq::Asset(ton_addr.to_string());
    let V1DexRsp::Asset(rsp) = client.dex.v1_exec(&req).await? else { panic!("AssetRsp expected") };
    assert_eq!(rsp.asset.contract_address, ton_addr);
    assert_eq!(rsp.asset.display_name, "TON");
    Ok(())
}

#[tokio::test]
async fn test_get_pools() -> Result<()> {
    let client = init_env();
    let req = V1DexReq::Pools(PoolsParams { dex_v2: true });
    let V1DexRsp::Pools(rsp) = client.dex.v1_exec(&req).await? else { panic!("PoolsRsp expected") };
    assert_ne!(rsp.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_get_pool() -> Result<()> {
    let client = init_env();
    let hmstr_usdt_pool = "EQBXg9I5MBvwv7O8Xd0ZOC6z7T6yoCojaBXQXoAYx6paDO2s";
    let req = V1DexReq::Pool(hmstr_usdt_pool.to_string());
    let V1DexRsp::Pool(rsp) = client.dex.v1_exec(&req).await? else { panic!("PoolRsp expected") };
    assert_eq!(rsp.pool.address, hmstr_usdt_pool);
    assert_eq!(rsp.pool.token0_address, "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo"); // hmstr
    assert_eq!(rsp.pool.token1_address, "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs"); // usdt
    Ok(())
}

#[tokio::test]
async fn test_get_routers() -> Result<()> {
    let client = init_env();
    let req = V1DexReq::Routers(RoutersParams { dex_v2: true });
    let V1DexRsp::Routers(rsp) = client.dex.v1_exec(&req).await? else { panic!("RoutersRsp expected") };
    assert_ne!(rsp.router_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_get_router() -> Result<()> {
    let client = init_env();
    let addr = "EQB3ncyBUTjZUA5EnFKR5_EnOMI9V1tTEAAPaiU71gc4TiUt";
    let req = V1DexReq::Router(addr.to_string());
    let V1DexRsp::Router(rsp) = client.dex.v1_exec(&req).await? else { panic!("RouterRsp expected") };
    assert_eq!(rsp.router.address, addr);
    Ok(())
}

#[tokio::test]
async fn test_swap_simulate() -> Result<()> {
    let client = init_env();

    let params = SwapSimulateParams {
        offer_address: "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo".to_string(), // hmstr
        ask_address: "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs".to_string(),   // usdt
        units: "40000000".to_string(),
        slippage_tolerance: "0.001".to_string(),
        referral_address: None,
        referral_fee: None,
        dex_v2: Some(true),
    };
    let req = V1DexReq::SwapSimulate(params.clone());
    let V1DexRsp::SwapSimulate(rsp) = client.dex.v1_exec(&req).await? else { panic!("SwapSimulateRsp expected") };
    assert_eq!(rsp.offer_address, params.offer_address);
    assert_eq!(rsp.ask_address, params.ask_address);
    Ok(())
}
