# AGENTS.md

## Scope

This crate is `dedust_api_client`, a Rust library crate that wraps DeDust REST
API v2.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `DedustApiClient::builder().build()?`
- `client.exec_api_v2(&V2Req::...)`
- request params in `api_v2/req.rs`
- response enums and models in `api_v2/rsp.rs` and `api_v2/types.rs`

Keep DeDust-specific address formatting and endpoint mapping in this crate.

## Public API Boundary

Treat these as public contracts:

- `DedustApiClient`
- `DEFAULT_API_V2_URL`
- `V2Req`
- `RoutingPlanParams`
- `V2Rsp` and public response/type structs
- `unwrap_rsp!`

`RoutingPlanParams::new` maps the zero TON address to `native` and all other
addresses to `jetton:<address>`. Do not change that mapping without validating
DeDust API expectations and updating examples.

## Live API Notes

Tests in `tests/test_api_v2.rs` hit the live DeDust API. Prefer assertions that
prove endpoint support and response parsing without relying on volatile pool
counts, routing amounts, or ordering.

## Downstream Integration Example

```rust
use dedust_api_client::api_v2::{RoutingPlanParams, V2Req, V2Rsp};
use dedust_api_client::client::DedustApiClient;

# async fn example() -> anyhow::Result<()> {
let client = DedustApiClient::builder().build()?;
let params = RoutingPlanParams::new(
    "0:0000000000000000000000000000000000000000000000000000000000000000",
    "0:0000000000000000000000000000000000000000000000000000000000000000",
    "1000000000",
);
let response = client.exec_api_v2(&V2Req::RoutingPlan(params)).await?;

match response {
    V2Rsp::RoutingPlan(routes) => println!("routes: {}", routes.len()),
    other => anyhow::bail!("unexpected DeDust response: {other:?}"),
}
# Ok(())
# }
```

Final applications should keep address, amount, and slippage interpretation in
their own domain layer.

## Validation

```bash
cargo test -p dedust_api_client --tests
cargo +nightly fmt
cargo clippy -p dedust_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p dedust_api_client --no-deps
cargo package --list -p dedust_api_client
```
