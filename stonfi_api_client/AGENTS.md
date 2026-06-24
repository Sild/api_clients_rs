# AGENTS.md

## Scope

This crate is `stonfi_api_client`, a Rust library crate that wraps STON.fi REST
API v1.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `StonfiApiClient::builder().build()?`
- `client.v1.exec(&V1Req::...)`
- request params in `v1/req.rs`
- response enums and models in `v1/rsp.rs`, `v1/types.rs`, and
  `v1/actions.rs`

Do not add application-level swap orchestration or transaction execution logic
here. This crate only wraps STON.fi API responses.

## Public API Boundary

Treat these as public contracts:

- `StonfiApiClient`
- `DEFAULT_API_V1_URL`
- `V1Client`
- `V1Req`
- all public request parameter structs
- `V1Rsp` and public response/action/type structs
- `unwrap_rsp!`

When adding an endpoint, update the README support matrix and add coverage that
proves the request path, method, params, and response parsing.

## Live API Notes

Tests in `tests/test_api.rs` hit the live STON.fi API. STON.fi asset metadata
and historical transaction fixtures can drift. Verify the live response before
changing client code.

Known recent contract observations:

- Native TON metadata has returned `display_name = "Gram"` and
  `symbol = "GRAM"`.
- Old transaction query fixtures may return no `tx_id`; avoid unwrapping
  fixture-dependent optional fields.

## Downstream Integration Example

```rust
use stonfi_api_client::client::StonfiApiClient;
use stonfi_api_client::v1::{PoolsParams, V1Req, V1Rsp};

# async fn example() -> anyhow::Result<()> {
let client = StonfiApiClient::builder().build()?;
let response = client.v1.exec(&V1Req::Pools(PoolsParams { dex_v2: true })).await?;

match response {
    V1Rsp::Pools(pools) => {
        println!("pools: {}", pools.pool_list.len());
    }
    other => {
        anyhow::bail!("unexpected STON.fi response: {other:?}");
    }
}
# Ok(())
# }
```

Final applications should treat values such as asset names, prices, transaction
status, and optional transaction IDs as live external data.

## Validation

```bash
cargo test -p stonfi_api_client --tests
cargo +nightly fmt
cargo clippy -p stonfi_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p stonfi_api_client --no-deps
cargo package --list -p stonfi_api_client
```
