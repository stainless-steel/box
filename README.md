# Box [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a storage for unique static strings.

## Example

```rust
use r#box::Symbol;

let one = Symbol::new("foo");
let other = Symbol::new("foo");
assert_eq!(one.as_ptr(), other.as_ptr());
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/stainless-steel/box/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/box/actions/workflows/build.yml
[documentation-img]: https://docs.rs/box/badge.svg
[documentation-url]: https://docs.rs/box
[package-img]: https://img.shields.io/crates/v/box.svg
[package-url]: https://crates.io/crates/box
