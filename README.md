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

Public request and response types are marked `#[non_exhaustive]` where the
workspace needs room to add fields or enum variants in future minor releases.
Use `Default::default()` or request parameter constructors instead of struct
literals, pass request parameters directly to clients where `Into<Request>` is
implemented, and keep wildcard arms when matching public enums.

Integration tests call the real upstream services and are intentionally not run
in CI/CD. Run live checks manually before pushing to a remote:

```bash
cargo test --workspace --tests
```
