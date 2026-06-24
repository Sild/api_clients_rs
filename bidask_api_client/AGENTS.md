# AGENTS.md

## Scope

This crate is `bidask_api_client`, a Rust library crate that wraps the Bidask
pools REST API.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `BidaskApiClient::builder().build()?`
- `client.exec_api(&Request::...)`
- request enum in `api/request.rs`
- response enum and models in `api/response.rs` and `api/types.rs`

The current client aggregates paginated `/pools` responses into one
`Vec<PoolInfo>`.

## Public API Boundary

Treat these as public contracts:

- `BidaskApiClient`
- `DEFAULT_API_URL`
- `Request`
- `Response`
- public response/type structs
- `unwrap_response!`

Public request/response types are `#[non_exhaustive]`; downstream matches need
wildcard arms, and examples should avoid struct literal construction for public
models.

Preserve pagination behavior unless the user explicitly asks for page-level
control. If adding endpoints, keep pagination decisions explicit in the request
or response type.

## Live API Notes

Tests in `tests/test_api.rs` hit the live Bidask API. Avoid brittle assertions
on pool totals, ordering, or token metadata. Do not replace live coverage with
mock-server tests.

## Downstream Integration Example

```rust
use bidask_api_client::api::{Request, Response};
use bidask_api_client::api_client::BidaskApiClient;

# async fn example() -> anyhow::Result<()> {
let client = BidaskApiClient::builder().build()?;
let response = client.exec_api(&Request::Pools).await?;

match response {
    Response::Pools(pools) => println!("pools: {}", pools.len()),
    other => anyhow::bail!("unexpected Bidask response: {other:?}"),
}
# Ok(())
# }
```

Final applications should apply their own filtering, persistence, and refresh
strategy after receiving the aggregated pool list.

## Validation

```bash
cargo test -p bidask_api_client --tests
cargo +nightly fmt
cargo clippy -p bidask_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p bidask_api_client --no-deps
cargo package --list -p bidask_api_client
```
