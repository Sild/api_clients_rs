use anyhow::Result;
use api_clients_core::ApiClientsError;
use stonfi_api_client::client::StonfiApiClient;
use stonfi_api_client::unwrap_response;
use stonfi_api_client::v1::*;

fn init_env() -> StonfiApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    StonfiApiClient::builder().build().unwrap()
}

#[tokio::test]
async fn test_swap_simulate_new_defaults_to_dex_v2() -> Result<()> {
    let params = SwapSimulateParams::new(
        "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo",
        "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs",
        "40000000",
        "0.001",
    );
    assert_eq!(params.dex_v2, Some(true));
    Ok(())
}

#[tokio::test]
async fn test_unsupported_request_returns_error() -> Result<()> {
    let client = init_env();
    let err = client.v1.exec(&V1Request::Markets).await.unwrap_err();
    match err {
        ApiClientsError::InvalidArgs(message) => assert!(message.contains("Markets")),
        other => anyhow::bail!("unexpected error: {other:?}"),
    }
    Ok(())
}

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_env();
    let request = V1Request::Assets;
    let response = unwrap_response!(Assets, client.v1.exec(&request).await?)?;
    assert_ne!(response.asset_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_assets_query() -> Result<()> {
    let client = init_env();
    let response = unwrap_response!(
        AssetsQuery,
        client
            .v1
            .exec(
                AssetsQueryParams::new()
                    .with_condition("!asset:liquidity:no & !asset:blacklisted".to_string())
                    .with_unconditional_assets(vec![]),
            )
            .await?
    )?;
    assert!(response.asset_list.len() > 1);

    let response = unwrap_response!(
        AssetsQuery,
        client
            .v1
            .exec(AssetsQueryParams::new().with_unconditional_assets(vec![
                "EQBfkgKxD8zkHquKL6pqZWGiQCkrbgXIw4ToqNRb9-RW0ba1".to_string(),
                "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs".to_string(),
            ]))
            .await?
    )?;
    assert_eq!(response.asset_list.len(), 3);
    Ok(())
}

#[tokio::test]
async fn test_asset() -> Result<()> {
    let client = init_env();
    let ton_addr = "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c";
    let request = V1Request::Asset(ton_addr.to_string());
    let response = unwrap_response!(Asset, client.v1.exec(&request).await?)?;
    assert_eq!(response.asset.contract_address, ton_addr);
    assert_eq!(response.asset.display_name, Some("Gram".to_string()));
    assert_eq!(response.asset.symbol, "GRAM");
    Ok(())
}

#[tokio::test]
async fn test_farms() -> Result<()> {
    let client = init_env();
    let response = unwrap_response!(Farms, client.v1.exec(FarmsParams::default()).await?)?;
    assert_ne!(response.farm_list, vec![]);

    Ok(())
}

#[tokio::test]
async fn test_farm() -> Result<()> {
    let client = init_env();
    let farm_minter = "EQCuq6v9szR4MrtVYN_kGHh2WMKU2ahJQzCL4J1VwfL1LSJm".to_string();
    let request = V1Request::Farm(farm_minter.clone());
    let response = unwrap_response!(Farm, client.v1.exec(&request).await?)?;
    assert_eq!(response.farm.minter_address, farm_minter);
    Ok(())
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let response = unwrap_response!(Pools, client.v1.exec(PoolsParams::default()).await?)?;
    assert_ne!(response.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools_by_market() -> Result<()> {
    let client = init_env();
    let params = PoolsByMarketParams::new(
        "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c", // ton
        "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs", // usdt
    );
    let response = unwrap_response!(Pools, client.v1.exec(params).await?)?;
    assert_ne!(response.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pool() -> Result<()> {
    let client = init_env();
    let hmstr_usdt_pool = "EQBXg9I5MBvwv7O8Xd0ZOC6z7T6yoCojaBXQXoAYx6paDO2s";
    let request = V1Request::Pool(hmstr_usdt_pool.to_string());
    let response = unwrap_response!(Pool, client.v1.exec(&request).await?)?;
    assert_eq!(response.pool.address, hmstr_usdt_pool);
    assert_eq!(response.pool.token0_address, "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo"); // hmstr
    assert_eq!(response.pool.token1_address, "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs"); // usdt
    Ok(())
}

#[tokio::test]
async fn test_routers() -> Result<()> {
    let client = init_env();
    let response = unwrap_response!(Routers, client.v1.exec(RoutersParams::default()).await?)?;
    assert_ne!(response.router_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_router() -> Result<()> {
    let client = init_env();
    let addr = "EQB3ncyBUTjZUA5EnFKR5_EnOMI9V1tTEAAPaiU71gc4TiUt";
    let request = V1Request::Router(addr.to_string());
    let response = unwrap_response!(Router, client.v1.exec(&request).await?)?;
    assert_eq!(response.router.address, addr);
    Ok(())
}

#[tokio::test]
async fn test_swap_simulate() -> Result<()> {
    let client = init_env();

    let params = SwapSimulateParams::new(
        "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo", // hmstr
        "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs", // usdt
        "40000000",
        "0.001",
    );
    let response = unwrap_response!(SwapSimulate, client.v1.exec(params.clone()).await?)?;
    assert_eq!(response.offer_address, params.offer_address);
    assert_eq!(response.ask_address, params.ask_address);
    Ok(())
}

#[tokio::test]
async fn test_tx_query() -> Result<()> {
    let client = init_env();
    let ext_msg_hash = "c88578dba0219474bc9eced97a5ed3c85d1bf1d48d4484832eb72113e982d181";
    let response = unwrap_response!(
        TransactionQuery,
        client
            .v1
            .exec(
                TransactionQueryParams::new()
                    .with_ext_msg_hash(ext_msg_hash.to_string())
                    .with_min_tx_timestamp("2025-11-25T00:00:00".to_string()),
            )
            .await?
    )?;

    assert!(response.tx_id.is_none());
    assert!(response.wallet_seqno.is_none());

    Ok(())
}

#[tokio::test]
async fn test_tx_action_tree_swap() -> Result<()> {
    let client = init_env();
    let hash = "0a3154df02213c26a88714c7f75b70701d8230f1ed0acf0d96fcea7458a723fc";
    let request = V1Request::TransactionActionTree(hash.to_string());
    let response = unwrap_response!(TransactionActionTree, client.v1.exec(&request).await?)?;

    assert_eq!(response.initial_tx_id.hash, hash);

    let swap_ask_jetton_amount = response.actions.first().and_then(|action| match action {
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
    let request = V1Request::TransactionActionTree(hash.to_string());
    let response = unwrap_response!(TransactionActionTree, client.v1.exec(&request).await?)?;

    assert_eq!(response.initial_tx_id.hash, hash);

    let lp_out = response.actions.get(1).and_then(|action| match action {
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
