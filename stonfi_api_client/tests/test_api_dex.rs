use anyhow::Result;
use stonfi_api_client::api_v1::dex_req::*;
use stonfi_api_client::api_v1::dex_rsp::V1DexRsp;
use stonfi_api_client::client::{StonfiApiClient, StonfiApiClientConfig};
use stonfi_api_client::unwrap_rsp;

fn init_env() -> StonfiApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let config = StonfiApiClientConfig {
        retry_count: 5,
        ..Default::default()
    };
    StonfiApiClient::new_with_config(config)
}

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_env();
    let req = V1DexReq::Assets;
    let rsp = unwrap_rsp!(Assets, client.v1_dex.exec(&req).await?)?;
    assert_ne!(rsp.asset_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_assets_query() -> Result<()> {
    let client = init_env();
    let req = V1DexReq::AssetsQuery(AssetsQueryParams {
        condition: Some("!asset:liquidity:no & !asset:blacklisted".to_string()),
        unconditional_asset: vec![],
        wallet_address: None,
    });
    let rsp = unwrap_rsp!(AssetsQuery, client.v1_dex.exec(&req).await?)?;
    assert_ne!(rsp.asset_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_asset() -> Result<()> {
    let client = init_env();
    let ton_addr = "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c";
    let req = V1DexReq::Asset(ton_addr.to_string());
    let rsp = unwrap_rsp!(Asset, client.v1_dex.exec(&req).await?)?;
    assert_eq!(rsp.asset.contract_address, ton_addr);
    assert_eq!(rsp.asset.display_name, Some("TON".to_string()));
    Ok(())
}

#[tokio::test]
async fn test_farms() -> Result<()> {
    let client = init_env();
    let params = FarmsParams { dex_v2: true };
    let req = V1DexReq::Farms(params);
    let rsp = unwrap_rsp!(Farms, client.v1_dex.exec(&req).await?)?;
    assert_ne!(rsp.farm_list, vec![]);

    Ok(())
}

#[tokio::test]
async fn test_farm() -> Result<()> {
    let client = init_env();
    let farm_minter = "EQCuq6v9szR4MrtVYN_kGHh2WMKU2ahJQzCL4J1VwfL1LSJm".to_string();
    let req = V1DexReq::Farm(farm_minter.clone());
    let rsp = unwrap_rsp!(Farm, client.v1_dex.exec(&req).await?)?;
    assert_eq!(rsp.farm.minter_address, farm_minter);
    Ok(())
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let req = V1DexReq::Pools(PoolsParams { dex_v2: true });
    let rsp = unwrap_rsp!(Pools, client.v1_dex.exec(&req).await?)?;
    assert_ne!(rsp.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools_by_market() -> Result<()> {
    let client = init_env();
    let params = PoolsByMarketParams {
        asset0_address: "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c".to_string(), // ton
        asset1_address: "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs".to_string(), // usdt
    };
    let req = V1DexReq::PoolsByMarket(params);
    let rsp = unwrap_rsp!(Pools, client.v1_dex.exec(&req).await?)?;
    assert_ne!(rsp.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pool() -> Result<()> {
    let client = init_env();
    let hmstr_usdt_pool = "EQBXg9I5MBvwv7O8Xd0ZOC6z7T6yoCojaBXQXoAYx6paDO2s";
    let req = V1DexReq::Pool(hmstr_usdt_pool.to_string());
    let rsp = unwrap_rsp!(Pool, client.v1_dex.exec(&req).await?)?;
    assert_eq!(rsp.pool.address, hmstr_usdt_pool);
    assert_eq!(rsp.pool.token0_address, "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo"); // hmstr
    assert_eq!(rsp.pool.token1_address, "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs"); // usdt
    Ok(())
}

#[tokio::test]
async fn test_routers() -> Result<()> {
    let client = init_env();
    let req = V1DexReq::Routers(RoutersParams { dex_v2: true });
    let rsp = unwrap_rsp!(Routers, client.v1_dex.exec(&req).await?)?;
    assert_ne!(rsp.router_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_router() -> Result<()> {
    let client = init_env();
    let addr = "EQB3ncyBUTjZUA5EnFKR5_EnOMI9V1tTEAAPaiU71gc4TiUt";
    let req = V1DexReq::Router(addr.to_string());
    let rsp = unwrap_rsp!(Router, client.v1_dex.exec(&req).await?)?;
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
    let rsp = unwrap_rsp!(SwapSimulate, client.v1_dex.exec(&req).await?)?;
    assert_eq!(rsp.offer_address, params.offer_address);
    assert_eq!(rsp.ask_address, params.ask_address);
    Ok(())
}
