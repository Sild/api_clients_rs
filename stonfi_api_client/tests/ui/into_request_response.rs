use stonfi_api_client::v1::{AssetsResponse, PoolsParams, V1Client, V1Request, V1Response};

async fn check_exec_accepts_into_request(client: V1Client) {
    let _ = client.exec(PoolsParams::default()).await;
}

fn check_from_variants() {
    let request: V1Request = PoolsParams::default().into();
    let _: V1Request = (&request).into();

    let assets: AssetsResponse = serde_json::from_str(r#"{"asset_list":[]}"#).unwrap();
    let _: V1Response = assets.into();
}

fn main() {}
