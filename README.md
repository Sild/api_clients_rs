# api_clients_rs
Rust bindings for different services. MRs are welcome!

| Service                          | Client                                             |
|----------------------------------|----------------------------------------------------|
| https://ston.fi                  | [stonfi_api_client](stonfi_api_client)             |
| https://dedust.io                | [dedust_api_client](dedust_api_client)             |
| https://app.tonco.io/            | [tonco_api_client](tonco_api_client)               |
| https://swap.coffee              | [swap_coffee_api_client](swap_coffee_api_client)   |
| https://api.bidask.finance/api   | [bidask_api_client](bidask_api_client)             |


If you're interested in some particular endpoint which is not implemented yet, just raise an issue and I'll add it.

## Usage

Add the crate for the service you need:

```toml
[dependencies]
stonfi_api_client = "0.8"
```

Then build a client and execute the typed request in an async Tokio runtime:

```rust
use stonfi_api_client::api_client::StonfiApiClient;
use stonfi_api_client::v1::{PoolsParams, V1Response};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = StonfiApiClient::builder().build()?;
let response = client.v1.exec(PoolsParams::default()).await?;

match response {
    V1Response::Pools(pools) => println!("pools: {}", pools.pool_list.len()),
    _ => println!("unexpected response variant"),
}
# Ok(())
# }
```

Public request and response types are marked `#[non_exhaustive]` where the
workspace needs room to add fields or enum variants in future minor releases.
Use `Default::default()` or request parameter constructors instead of struct
literals, pass request parameters directly to clients where `Into<Request>` is
implemented, and keep wildcard arms when matching public enums.

All service clients use `api_clients_core::Executor` underneath. The default
executor retries transient failures, uses a 10-second timeout, and applies a
smooth 10 RPS client-side rate limit. Applications that need a different
transport policy can build an `Executor` directly and inject it through the
service client builder.

Integration tests call the real upstream services and are intentionally not run
in CI/CD. Run live checks manually before pushing to a remote:

```bash
cargo test --workspace --tests
```
