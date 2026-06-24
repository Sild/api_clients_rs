### https://api.ston.fi/swagger-ui
| Group | Method                                        | Supported |
|-------|-----------------------------------------------|----------|
| AMM   | /v1/assets                                    | ✅        |
| AMM   | /v1/assets/query                              | ✅        |
| AMM   | /v1/assets/search                             | ✅        |
| AMM   | /v1/assets/{addr_str}                         | ✅        |
| AMM   | /v1/farms                                     | ✅        |
| AMM   | /v1/farms/by_pool/{pool_addr_str}             | ✅        |
| AMM   | /v1/farms/{addr_str}                          | ✅        |
| AMM   | /v1/liquidity_provision/simulate              |          |
| AMM   | /v1/markets                                   | ✅        |
| AMM   | /v1/pools/query                               | ✅        |
| AMM   | /v1/pools                                     | ✅        |
| AMM   | /v1/pools/by_market/{asset0_str}/{asset1_str} | ✅        |
| AMM   | /v1/pools/{addr_str}                          | ✅        |
| AMM   | /v1/reverse_swap/simulate                     | ✅        |
| AMM   | /v1/routers                                   | ✅        |
| AMM   | /v1/routers/{addr_str}                        | ✅        |
| AMM   | /v1/stats/dex                                 | ✅        |
| AMM   | /v1/stats/fee_accruals                        | ✅        |
| AMM   | /v1/stats/fee_withdrawals                     | ✅        |
| AMM   | /v1/stats/fees                                | ✅        |
| AMM   | /v1/stats/operations                          | ✅        |
| AMM   | /v1/stats/pool                                | ✅        |
| AMM   | /v1/stats/staking                             | ✅        |
| AMM   | /v1/swap/simulate                             | ✅        |
| AMM   | /v1/swap/status                               | ✅        |
| AMM   | /v1/transactions/query                        | ✅        |
| AMM   | /v1/transactions/{hash}/action_tree           | ✅        |

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Build request parameter structs through `Default::default()` or
`new()` constructors, pass them directly to `V1Client::exec` where `Into<V1Request>`
is implemented, and include a wildcard arm when matching response enums.
