use stonfi_api_client::unwrap_response;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response: stonfi_api_client::v1::V1Response =
        serde_json::from_str(r#"{"Assets":{"asset_list":[]}}"#)?;
    let _assets = unwrap_response!(Assets, response)?;
    Ok(())
}
