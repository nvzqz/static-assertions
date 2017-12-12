# static_assertions [![Crates.io][crate-badge] ![Downloads][crate-dl]][crate] [![Build Status][travis-badge]][travis]

Rust compile-time assertions to ensure that invariants are met.

[Documentation][crate-doc]

## Installation

This crate is available [on crates.io][crate] and can be used by adding the
following to your project's `Cargo.toml`:

```toml
[dependencies]
static_assertions = "0.2.5"
```

and this to your crate root:

```rust
#[macro_use]
extern crate static_assertions;
```

## Usage

### Assert Equal Size

Use `assert_eq_size!` to ensure two types are the same size:

```rust
// Requires a label if in module scope
assert_eq_size!(byte; u8, u8);

fn func() {
    // If label-less, must be in a function to work
    assert_eq_size!([u8; 4], u32);

    // Supports unlimited arguments
    assert_eq_size!([u8; 8], u64, (u32, u32), (u32, u16, u16), ...);

    // Fails to compile
    assert_eq_size!(u16, u64);
}

```

Use `assert_eq_size_val!` to ensure two values are the same size:

```rust
let x: u32 = 42;
let y: u32 = 10;
assert_eq_size_val!(x, y, [0u8; 4]);

// Fails to compile
assert_eq_size_val!(x, 0u8);
```

_**Note:** Both macros support multiple arguments and are not restricted by the recursion limit._

### Assert Constant Expression

A constant expression can be ensured to evaluate to `true` at compile-time.

The `const_assert` and `const_assert_eq` macros have the same scope and label
limitations as `assert_eq_size`.

```rust
// Supports constants
const FIVE: usize = 5;

fn func() {
    const_assert!(1 + 1 == 2);

    // Supports unlimited comma-separated conditions
    const_assert!(4 > 3, 3 + 2 == FIVE);

    // Fails to compile
    const_assert!(2 != 2);
}
```

### Limitations

See [issue #1](https://github.com/nvzqz/static-assertions-rs/issues/1) to read
up on current limitations of this crate and how to currently overcome them.

## License

This project is released under either:

- [MIT License][license-mit]

- [Apache License (Version 2.0)][license-apache]

at your choosing.

[crate]:       https://crates.io/crates/static_assertions
[crate-dl]:    https://img.shields.io/crates/d/static_assertions.svg
[crate-doc]:   https://docs.rs/static_assertions/
[crate-badge]: https://img.shields.io/crates/v/static_assertions.svg

[travis]:       https://travis-ci.org/nvzqz/static-assertions-rs
[travis-badge]: https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master

[license-mit]:    https://github.com/nvzqz/static-assertions-rs/blob/master/LICENSE-MIT
[license-apache]: https://github.com/nvzqz/static-assertions-rs/blob/master/LICENSE-APACHE
