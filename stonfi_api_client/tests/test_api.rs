use anyhow::Result;
use api_clients_core::ApiClientsError;
use stonfi_api_client::client::StonfiApiClient;
use stonfi_api_client::unwrap_rsp;
use stonfi_api_client::v1::V1Rsp;
use stonfi_api_client::v1::*;

fn init_env() -> StonfiApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    StonfiApiClient::builder().build().unwrap()
}

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_env();
    let req = V1Req::Assets;
    let rsp = unwrap_rsp!(Assets, client.v1.exec(&req).await?)?;
    assert_ne!(rsp.asset_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_assets_query() -> Result<()> {
    let client = init_env();
    let req = V1Req::AssetsQuery(AssetsQueryParams {
        condition: Some("!asset:liquidity:no & !asset:blacklisted".to_string()),
        unconditional_assets: vec![],
        wallet_address: None,
    });
    let rsp = unwrap_rsp!(AssetsQuery, client.v1.exec(&req).await?)?;
    assert!(rsp.asset_list.len() > 1);

    let req = V1Req::AssetsQuery(AssetsQueryParams {
        condition: None,
        unconditional_assets: vec![
            "EQBfkgKxD8zkHquKL6pqZWGiQCkrbgXIw4ToqNRb9-RW0ba1".to_string(),
            "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs".to_string(),
        ],
        wallet_address: None,
    });
    let rsp = unwrap_rsp!(AssetsQuery, client.v1.exec(&req).await?)?;
    assert_eq!(rsp.asset_list.len(), 3);
    Ok(())
}

#[tokio::test]
async fn test_asset() -> Result<()> {
    let client = init_env();
    let ton_addr = "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c";
    let req = V1Req::Asset(ton_addr.to_string());
    let rsp = unwrap_rsp!(Asset, client.v1.exec(&req).await?)?;
    assert_eq!(rsp.asset.contract_address, ton_addr);
    assert_eq!(rsp.asset.display_name, Some("TON".to_string()));
    Ok(())
}

#[tokio::test]
async fn test_farms() -> Result<()> {
    let client = init_env();
    let params = FarmsParams { dex_v2: true };
    let req = V1Req::Farms(params);
    let rsp = unwrap_rsp!(Farms, client.v1.exec(&req).await?)?;
    assert_ne!(rsp.farm_list, vec![]);

    Ok(())
}

#[tokio::test]
async fn test_farm() -> Result<()> {
    let client = init_env();
    let farm_minter = "EQCuq6v9szR4MrtVYN_kGHh2WMKU2ahJQzCL4J1VwfL1LSJm".to_string();
    let req = V1Req::Farm(farm_minter.clone());
    let rsp = unwrap_rsp!(Farm, client.v1.exec(&req).await?)?;
    assert_eq!(rsp.farm.minter_address, farm_minter);
    Ok(())
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let req = V1Req::Pools(PoolsParams { dex_v2: true });
    let rsp = unwrap_rsp!(Pools, client.v1.exec(&req).await?)?;
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
    let req = V1Req::PoolsByMarket(params);
    let rsp = unwrap_rsp!(Pools, client.v1.exec(&req).await?)?;
    assert_ne!(rsp.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pool() -> Result<()> {
    let client = init_env();
    let hmstr_usdt_pool = "EQBXg9I5MBvwv7O8Xd0ZOC6z7T6yoCojaBXQXoAYx6paDO2s";
    let req = V1Req::Pool(hmstr_usdt_pool.to_string());
    let rsp = unwrap_rsp!(Pool, client.v1.exec(&req).await?)?;
    assert_eq!(rsp.pool.address, hmstr_usdt_pool);
    assert_eq!(rsp.pool.token0_address, "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo"); // hmstr
    assert_eq!(rsp.pool.token1_address, "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs"); // usdt
    Ok(())
}

#[tokio::test]
async fn test_routers() -> Result<()> {
    let client = init_env();
    let req = V1Req::Routers(RoutersParams { dex_v2: true });
    let rsp = unwrap_rsp!(Routers, client.v1.exec(&req).await?)?;
    assert_ne!(rsp.router_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_router() -> Result<()> {
    let client = init_env();
    let addr = "EQB3ncyBUTjZUA5EnFKR5_EnOMI9V1tTEAAPaiU71gc4TiUt";
    let req = V1Req::Router(addr.to_string());
    let rsp = unwrap_rsp!(Router, client.v1.exec(&req).await?)?;
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
    let req = V1Req::SwapSimulate(params.clone());
    let rsp = unwrap_rsp!(SwapSimulate, client.v1.exec(&req).await?)?;
    assert_eq!(rsp.offer_address, params.offer_address);
    assert_eq!(rsp.ask_address, params.ask_address);
    Ok(())
}

#[tokio::test]
async fn test_tx_action_tree_swap() -> Result<()> {
    let client = init_env();
    let hash = "0a3154df02213c26a88714c7f75b70701d8230f1ed0acf0d96fcea7458a723fc";
    let req = V1Req::TransactionActionTree(hash.to_string());
    let rsp = unwrap_rsp!(TransactionActionTree, client.v1.exec(&req).await?)?;

    assert_eq!(rsp.initial_tx_id.hash, hash);

    let swap_ask_jetton_amount = rsp.actions.first().and_then(|action| match action {
        Action::Amm(AmmAction::Swap(data)) => match &data.status {
            ActionStatus::Completed(payload) => match &payload.data {
                SwapResult::Ok { ask_jetton_amount, .. } => Some(ask_jetton_amount.clone()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    });

    assert_eq!(swap_ask_jetton_amount, Some("23810546".to_string()));

    Ok(())
}

#[tokio::test]
async fn test_tx_action_tree_swap_plus_lp() -> Result<()> {
    let client = init_env();
    let hash = "ccd5d1c148ffc9157dd248b8350da05a17e958d913150fde3904e85e82c5b5a9";
    let req = V1Req::TransactionActionTree(hash.to_string());
    let rsp = unwrap_rsp!(TransactionActionTree, client.v1.exec(&req).await?)?;

    assert_eq!(rsp.initial_tx_id.hash, hash);

    let lp_out = rsp.actions.get(1).and_then(|action| match action {
        Action::Amm(AmmAction::ProvideLiquidity(data)) => match data.effects.first() {
            Some(ProvideLiquidityEffect::CbAddLiquidity(effect_data)) => match &effect_data.status {
                ActionStatus::Completed(payload) => match &payload.data {
                    CbAddLiquidityResult::MintLp { lp_out } => Some(lp_out.clone()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    });

    assert_eq!(lp_out, Some("85942".to_string()));

    Ok(())
}
