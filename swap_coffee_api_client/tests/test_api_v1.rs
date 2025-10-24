use api_clients_core::ApiClientError;
use std::vec;

use anyhow::Result;
use swap_coffee_api_client::api_v1::V1Req;
use swap_coffee_api_client::api_v1::{Dexes, V1Rsp};
use swap_coffee_api_client::client::SwapCoffeeApiClient;
use swap_coffee_api_client::unwrap_rsp;

fn init_env() -> SwapCoffeeApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    SwapCoffeeApiClient::builder().with_retry_count(5).build()
}

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_env();
    let req = V1Req::Assets;
    let rsp = unwrap_rsp!(Assets, client.exec_api_v1(&req).await?)?;
    assert_ne!(rsp, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let req = V1Req::Pools(Dexes {
        dexes: "coffee".to_string(),
    });
    let rsp = unwrap_rsp!(Pools, client.exec_api_v1(&req).await?)?;
    log::debug!("rsp: {:?}", rsp[0].pools[0]);
    assert_ne!(rsp[0].pools, vec![]);
    Ok(())
}
