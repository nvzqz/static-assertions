# static_assertions

Rust compile-time assertions.

[![Crates.io][crate-badge]][crate]

[![Build Status][travis-badge]][travis]

## Usage

### Assert Equal Size

Use `assert_eq_size!` to ensure two types are the same size:

```rust
assert_eq_size!([u8; 4], u32);
assert_eq_size!([u8; 8], u64, (u32, u32), (u32, u16, u16), ...);

// Fails to compile
assert_eq_size!(u16, u64);
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

**Limitation:** Due to implementation details, these macros can only be called
from within the context of a function. This may change when `mem::size_of`
becomes a `const fn`.

### Assert Constant Expression

A constant expression can be ensured to evaluate to `true` at compile-time.

```rust
const_assert!(1 + 1 == 2);

// Supports constants
const FIVE: usize = 5;

// Supports comma and semicolon-separated conditions
const_assert!(4 > 3, 3 + 2 == FIVE);
const_assert! {
    FIVE + FIVE == 10;
    FIVE / FIVE == 1;
}

// Fails to compile
const_assert!(2 != 2);
```

**Limitation:** Due to implementation details, `const_assert!` can only be
called from within the context of a function.

## License

This project is released under either:

- [MIT License][license-mit]

- [Apache License (Version 2.0)][license-apache]

at your choosing.

[crate]:       https://crates.io/crates/static_assertions
[crate-badge]: https://img.shields.io/crates/v/static_assertions.svg

[travis]:       https://travis-ci.org/nvzqz/static-assertions-rs
[travis-badge]: https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master

[license-mit]:    https://github.com/nvzqz/static-assertions-rs/blob/master/LICENSE-MIT
[license-apache]: https://github.com/nvzqz/static-assertions-rs/blob/master/LICENSE-APACHE
