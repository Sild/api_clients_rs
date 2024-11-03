use anyhow::Result;
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
    let rsp = client.get_pools().await?;
    assert_ne!(rsp, vec![]);

    Ok(())
}

#[tokio::test]
async fn test_get_pools_lite() -> Result<()> {
    let client = init_env();
    let rsp = client.get_pools_lite().await?;
    assert_ne!(rsp, vec![]);

    Ok(())
}
