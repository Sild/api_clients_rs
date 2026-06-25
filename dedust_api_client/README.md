### https://api.dedust.io/

| Method                                   | Supported |
|------------------------------------------|-----------|
| /v2/accounts/{address}/assets            |           |
| /v2/accounts/{address}/trades            |           |
| /v2/assets                               | ✅         |
| /v2/assets/{symbol}                      |           |
| /v2/coinmarketcap/markets                |           |
| /v2/dns/{domain}                         |           |
| /v2/gcko/pairs                           |           |
| /v2/gcko/tickers                         |           |
| /v2/gcko/trades                          |           |
| /v2/jettons/{address}/circulating-supply |           |
| /v2/jettons/{address}/metadata           |           |
| /v2/jettons/{address}/top-buys           |           |
| /v2/jettons/{address}/top-traders        |           |
| /v2/jettons/{address}/total-supply       |           |
| /v2/pools                                | ✅         |
| /v2/pools-lite                           | ✅         |
| /v2/pools/{address}/metadata             |           |
| /v2/pools/{address}/trades               | ✅         |
| /v2/prices                               |           |
| /v2/routing/plan                         | ✅         |

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Build public POD structs with `Default::default().with_<field>(...)`
or request parameter constructors, pass request parameters directly to
`exec_api_v2` where `Into<V2Request>` is implemented, and include a wildcard arm
when matching response enums.
