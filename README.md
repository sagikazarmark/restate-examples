# Restate examples

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sagikazarmark/restate-examples/dagger.yaml?style=flat-square)](https://github.com/sagikazarmark/restate-examples/actions/workflows/dagger.yaml)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/sagikazarmark/restate-examples/badge?style=flat-square)](https://securityscorecards.dev/viewer/?uri=github.com/sagikazarmark/restate-examples)

**Restate examples**

## Quickstart

TODO

## Configuration

TODO

## Deployment

TODO

## Development

Minimum verification:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`

Or run the same checks (fmt, clippy, test, doc, build, and endpoint registration) in a container with [Dagger](https://dagger.io), exactly as CI does:

- `dagger check`

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Some of the examples are based on the [official examples](https://github.com/restatedev/sdk-rust/tree/main/examples).
