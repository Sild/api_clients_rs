use api_clients_core::ApiClientsError;
use std::vec;

use anyhow::Result;
use bidask_api_client::api::Req;
use bidask_api_client::api::Rsp;
use bidask_api_client::client::BidaskApiClient;
use bidask_api_client::unwrap_rsp;

fn init_env() -> BidaskApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    BidaskApiClient::builder().build().unwrap()
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let req = Req::Pools;
    let rsp = unwrap_rsp!(Pools, client.exec_api(&req).await?)?;
    assert_ne!(rsp, vec![]);
    log::debug!("{:?}", rsp.len());
    Ok(())
}
