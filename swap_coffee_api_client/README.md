### https://backend.swap.coffee/v1

| Method                                   | Supported |
|------------------------------------------|-----------|
| /v1/pools                                | ✅        |
| /v1/tokens                               | ✅        |

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Build public POD structs with `Default::default().with_<field>(...)`
or request parameter constructors, pass request parameters directly to
`exec_api_v1` where `Into<V1Request>` is implemented, and include a wildcard arm
when matching response enums.
