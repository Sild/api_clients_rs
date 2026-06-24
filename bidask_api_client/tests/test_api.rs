use std::vec;

use anyhow::Result;
use bidask_api_client::api::Request;
use bidask_api_client::client::BidaskApiClient;
use bidask_api_client::unwrap_response;

fn init_env() -> BidaskApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    BidaskApiClient::builder().build().unwrap()
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let request = Request::Pools;
    let response = unwrap_response!(Pools, client.exec_api(&request).await?)?;
    assert_ne!(response, vec![]);
    log::debug!("{:?}", response.len());
    Ok(())
}
