# static_assertions

Rust compile-time assertions.

[![Build Status](https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master)](https://travis-ci.org/nvzqz/static-assertions-rs)

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

### Assert Constant Expression

A constant expression can be ensured to evaluate to `true` at compile-time.

**Limitation:** Due to implementation details, `const_assert!` can only be
called from within the context of a function.

```rust
const_assert!(1 + 1 == 2);

// Supports constants
const FIVE: usize = 5;

// Supports comma and semicolon-separated conditions
const_assert!(4 > 3, 3 + 2 == FIVE);
const_assert! {
    FIVE + FIVE == 10;
    FIVE / FIVE == 0;
}

// Fails to compile
const_assert!(2 != 2);
```

## License

This project is released under either:

- [MIT License][license-mit]

- [Apache License (Version 2.0)][license-apache]

at your choosing.

[license-mit]: https://github.com/nvzqz/static-assertions-rs/blob/master/LICENSE-MIT
[license-apache]: https://github.com/nvzqz/static-assertions-rs/blob/master/LICENSE-APACHE
