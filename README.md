# Bisector

## Overview

An implementation of the bisection method, aimed to be flexible in its usage. For example, the implementation is not limited
to numeric types, and does not hold internal state of the current step, allowing for the ability to re-execute a step.
The lack of internal state also allows the implementation to take a shared reference (`&self`), instead of an exclusive
reference (`&mut self`), which is convenient, when a process which also takes an exclusive reference must be executed
as part of the search. Lastly, the output of each step can completely be determined by the user.

## Example

This section will be fleshed out in the future. For now, an [example](https://github.com/foresterre/cargo-msrv/pull/260/files#diff-b07c759a3c8bb5d2b413cddb592fa05eee0419b97e0a947e6bd508e0f97624eaR67-R96) exists as part of a PR in [cargo-msrv](https://github.com/foresterre/cargo-msrv).

## Install

### Cargo

Add the `bisector` crate to list of dependencies: 
```toml
[dependencies]
bisector = "0.1.0"
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.