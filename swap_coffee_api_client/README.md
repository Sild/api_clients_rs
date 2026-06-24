### https://backend.swap.coffee/v1

| Method                                   | Supported |
|------------------------------------------|-----------|
| /v1/pools                                | ✅        |
| /v1/tokens                               | ✅        |

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Build request parameter structs through their `new()` constructors,
pass them directly to `exec_api_v1` where `Into<V1Request>` is implemented, and
include a wildcard arm when matching response enums.
