# intfloat
Fast and simple hashable float alternative using ints

## Installing

```sh
$ cargo add intfloat
```

Alternatively, you can edit your `Cargo.toml` directly and run `cargo update`:

```toml
[dependencies]
intfloat = "0.1.0"
```

## Getting started

IntFloat can be used as a fast hashable float alternative, with a limited accuracy compared to f32/f64. To use it, call
it with IntFloat::from(x, y), with x the float you want to convert and y the number of decimal points you want to keep.

```rust
use intfloat::IntFloat;
let a = IntFloat::from(10 as f32, 0);
let b = IntFloat::from(5.2, 0);
assert_eq!(a, b+b);

let c = IntFloat::from(5.2, 1);
assert_ne!(b, c);
```

Internally, the float is represented by two integers z and y, such that x = z * 10^(-y), so for example 
5.2 = 52 * 10 ^ -1. 

See the [docs](https://docs.rs/intervalues/) for more functions and details. Feel free to open an issue in case some
things are unclear.

## Motivation & goal
This package is implemented in the context of (and usable by) the package 
[intervalues](https://crates.io/crates/intervalues/). That package is used to track (combinations of) intervals between
points, for which an internal algorithm uses HashMaps. To support float as interval bounds as well, the input floats
need to be converted to a hashable float. An alternative to IntFloat is rust_decimal::Decimal, which is more mature
and feature-rich, but performs about 30% slower when used with intervalues::combine_intervals on large datasets, likely
due to the extra accuracy/granularity that Decimal can take into account.

This package aims to add a faster alternative when small discrepancies don't matter. For example, in many cases it is
fine to round a float to 4 decimal places, which can accurately be represented by IntFloat(..., 4).

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 