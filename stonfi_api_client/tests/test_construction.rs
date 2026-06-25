use stonfi_api_client::v1::{
    ActionData, Asset, AssetsResponse, Blank, Pool, PoolsByMarketParams, SwapData, SwapSimulateResponse, TxId,
};

#[test]
fn test_request_params_support_default_setter_construction() {
    let params = PoolsByMarketParams::default()
        .with_asset0_address("asset-0".to_string())
        .with_asset1_address("asset-1".to_string());

    assert_eq!(params.asset0_address, "asset-0");
    assert_eq!(params.asset1_address, "asset-1");
}

#[test]
fn test_response_models_support_default_setter_construction() {
    let asset = Asset::default().with_contract_address("asset".to_string()).with_symbol("ASSET".to_string());
    let response = AssetsResponse::default().with_asset_list(vec![asset.clone()]);

    assert_eq!(asset.contract_address, "asset");
    assert_eq!(response.asset_list, vec![asset]);
}

#[test]
fn test_action_models_support_default_setter_construction() {
    let tx_id = TxId::default().with_hash("hash".to_string());
    let data = ActionData::<Blank, SwapData, Blank>::default().with_destination("destination".to_string());
    let pool = Pool::default().with_address("pool".to_string());
    let simulation = SwapSimulateResponse::default().with_pool_address("pool".to_string());

    assert_eq!(tx_id.hash, "hash");
    assert_eq!(data.destination.as_deref(), Some("destination"));
    assert_eq!(pool.address, "pool");
    assert_eq!(simulation.pool_address, "pool");
}
