# AGENTS.md

## Scope

This crate is `swap_coffee_api_client`, a Rust library crate that wraps Swap
Coffee REST API v1.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `SwapCoffeeApiClient::builder().build()?`
- `client.exec_api_v1(&V1Req::...)`
- request params in `api_v1/req.rs`
- response enums and models in `api_v1/rsp.rs` and `api_v1/types.rs`

Do not add routing decisions or swap execution flows here. Keep this crate to
API request/response wrapping.

## Public API Boundary

Treat these as public contracts:

- `SwapCoffeeApiClient`
- `DEFAULT_API_V1_URL`
- `V1Req`
- `Dexes`
- `V1Rsp` and public response/type structs
- `unwrap_rsp!`

`Dexes` is serialized into query parameters for `/pools`; preserve that shape
unless the upstream API changes.

## Live API Notes

Tests in `tests/test_api_v1.rs` hit the live Swap Coffee API. Avoid brittle
assertions on returned pool counts, token ordering, or dynamic metadata.

## Downstream Integration Example

```rust
use swap_coffee_api_client::api_v1::{Dexes, V1Req, V1Rsp};
use swap_coffee_api_client::client::SwapCoffeeApiClient;

# async fn example() -> anyhow::Result<()> {
let client = SwapCoffeeApiClient::builder().build()?;
let response = client
    .exec_api_v1(&V1Req::Pools(Dexes {
        dexes: "stonfi_v2".to_string(),
    }))
    .await?;

match response {
    V1Rsp::Pools(pools) => println!("pool pages: {}", pools.len()),
    other => anyhow::bail!("unexpected Swap Coffee response: {other:?}"),
}
# Ok(())
# }
```

Final applications should keep routing preferences, slippage, and trade safety
checks outside this crate.

## Validation

```bash
cargo test -p swap_coffee_api_client --tests
cargo +nightly fmt
cargo clippy -p swap_coffee_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p swap_coffee_api_client --no-deps
cargo package --list -p swap_coffee_api_client
```
