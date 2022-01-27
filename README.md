# Bisector

## Overview

A flexible, stateless implementation of the [bisection method](https://en.wikipedia.org/wiki/Bisection_method). 

Flexibility is achieved by giving the user of this crate control over the input and output types. That is,
this implementation is not limited to numeric types. In addition, the implementation is stateless. The [Bisector](https://docs.rs/bisector/latest/bisector/struct.Bisector.html#)
struct on which the [bisect](https://docs.rs/bisector/latest/bisector/struct.Bisector.html#method.bisect) methods are implemented
does not hold internal mutable state of the last step. This gives the user the option to re-execute a step,
or really perform any step in the order the user desires (although incremental steps may be most logical still 😅).

The lack of internal mutable state also allows the implementation to take a shared reference (`&self`), instead of an exclusive
reference (`&mut self`), which is convenient when dealing with ownership in many cases, and was the original reason
behind this crate.

## Install

### Cargo

1) Last [published](https://crates.io/crates/bisector) version on crates.io (recommended):

Add the `bisector` crate to list of dependencies in your Cargo manifest (`Cargo.toml`):
```toml
[dependencies]
bisector = "*" # replace `*` with latest version
```

2) Last [development](https://github.com/foresterre/bisector) version on GitHub:

Add the `bisector` crate to list of dependencies in your Cargo manifest (`Cargo.toml`):
```toml
[dependencies]
bisector = { git = "https://github.com/foresterre/bisector.git" }
```

## MSRV

The Minimal Supported Rust Version was determined with [cargo-msrv](https://github.com/foresterre/cargo-msrv), and 
is verified on the during CI runs. The table below lists the MSRV for the current and historical versions of `bisector`. 

| `bisector version` | MSRV |
|--------------------|------|
| 0.1.0              | N/A  |
| 0.2.0              | N/A  |
| 0.3.0              | 1.37 |


## Examples

### Example 1

```rust
fn main() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this example, we'll manually step through the bisection (i.e. without a loop).

    // (1) We use the default starting indices (i.e. left = 0, right = |values| - 1 = 9);
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), start_from);

    // We converge to the left, so our view of values will be halved to the left half.
    assert_eq!(step.indices, Indices::new(0, 4));
    assert_eq!(step.result.unwrap().try_into_left().unwrap(), 5);

    // (2) Now we use the next indices produced by step, to progress our bisection: step.indices
    //      Because we zig-zag, we'll now converge to the right
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), step.indices);

    // We converge to the right, so our view of values will be halved to the right half of our previous
    // view.
    assert_eq!(step.indices, Indices::new(3, 4));
    assert_eq!(step.result.unwrap().try_into_right().unwrap(), 3);

    // (3) Step further: zig-zag left
    let final_step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    assert_eq!(final_step.indices, Indices::new(3, 3));
    assert_eq!(final_step.result.unwrap().try_into_left().unwrap(), 4);

    // (4a) Step a one more time to check we are at the end: left
    let step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Left(value), final_step.indices);

    assert_eq!(step.indices, Indices::new(3, 3));
    assert!(step.result.is_none());

    // (4b) Step a one more time to check we are at the end: right
    let step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Right(value), final_step.indices);

    assert_eq!(step.indices, Indices::new(3, 3));
    assert!(step.result.is_none());
}
```

### Example 2

```rust
// NB: output held by ConvergeTo does *not* need to be of the same type as
// the value. In this example, it just happens to be the case.
fn f(value: u32) -> ConvergeTo<u32, u32> {
    if value >= 5 && value <= 6 {
        ConvergeTo::Right(value)
    } else {
        ConvergeTo::Left(value)
    }
}

fn main() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let bisector = Bisector::new(&values);
    let mut elements_seen = vec![];
    let mut value = None;

    let mut i = Indices::from_bisector(&bisector);
    while let Step {
        indices,
        result: Some(t),
    } = bisector.bisect(|&v| f(v), i)
    {
        i = indices;

        let val = match t {
            ConvergeTo::Left(l) => l,
            ConvergeTo::Right(r) => r,
        };

        elements_seen.push(val);
        value = Some(val);
    }

    println!("{:?}", elements_seen);
    println!("Final converged to '{}'", value.unwrap());
}
```

### Example: bisect in [cargo msrv](https://github.com/foresterre/cargo-msrv)

A more contrived [example](https://github.com/foresterre/cargo-msrv/blob/6c18525f4c1dcb888b6e4392cef52c8ecdf1adc6/src/search_methods/bisect.rs) can be found in `cargo msrv`.

NB: Linked revision was implemented before [Bisector::try_bisect](https://docs.rs/bisector/latest/bisector/struct.Bisector.html#method.try_bisect) was added. 
To cover a fallible case in the convergence function, you may want to use [Bisector::try_bisect](https://docs.rs/bisector/latest/bisector/struct.Bisector.html#method.try_bisect)
over [Bisector::bisect](https://docs.rs/bisector/latest/bisector/struct.Bisector.html#method.bisect).

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