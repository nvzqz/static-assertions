[![Banner](https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Banner.png)](https://github.com/nvzqz/static-assertions-rs)

<div align="center">
    <a href="https://crates.io/crates/static_assertions">
        <img src="https://img.shields.io/crates/v/static_assertions.svg" alt="Crates.io">
        <img src="https://img.shields.io/crates/d/static_assertions.svg" alt="Downloads">
    </a>
    <a href="https://travis-ci.org/nvzqz/static-assertions-rs">
        <img src="https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master" alt="Build Status">
    </a>
    <img src="https://img.shields.io/badge/rustc-^1.37.0-blue.svg" alt="rustc ^1.37.0">
    <br>
    <a href="https://www.patreon.com/nvzqz">
        <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
    </a>
    <a href="https://www.paypal.me/nvzqz">
        <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
    </a>
</div>

Rust compile-time assertions to ensure that invariants are met.

[Documentation](https://docs.rs/static_assertions/)

## Installation

This crate is available
[on crates.io](https://crates.io/crates/static_assertions) and can be used by
adding the following to your project's
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
static_assertions = "1.0.0"
```

and this to your crate root (`main.rs` or `lib.rs`):

```rust
#[macro_use]
extern crate static_assertions;
```

## Usage

This crate exposes the following macros:
- [`assert_cfg!`]
- [`assert_eq_align!`]
- [`assert_eq_size!`]
- [`assert_eq_size_ptr!`]
- [`assert_eq_size_val!`]
- [`assert_fields!`]
- [`assert_impl_all!`]
- [`assert_not_impl_all!`]
- [`assert_not_impl_any!`]
- [`assert_obj_safe!`]
- [`assert_type_eq_all!`]
- [`assert_type_ne_all!`]
- [`const_assert!`]
- [`const_assert_eq!`]
- [`const_assert_ne!`]

## Changes

See [`CHANGELOG.md`](https://github.com/nvzqz/static-assertions-rs/blob/master/CHANGELOG.md)
for a complete list of what has changed from one version to another.

## License

This project is released under either:

- [MIT License](https://github.com/nvzqz/static-assertions-rs/blob/master/LICENSE-MIT)

- [Apache License (Version 2.0)](https://github.com/nvzqz/static-assertions-rs/blob/master/LICENSE-APACHE)

at your choosing.

[new issue]: https://github.com/nvzqz/static-assertions-rs/issues/new

[`assert_cfg!`]:          https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_cfg.html
[`assert_eq_align!`]:     https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_eq_align.html
[`assert_eq_size!`]:      https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_eq_size.html
[`assert_eq_size_ptr!`]:  https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_eq_size_ptr.html
[`assert_eq_size_val!`]:  https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_eq_size_val.html
[`assert_fields!`]:       https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_fields.html
[`assert_impl_all!`]:     https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_impl_all.html
[`assert_not_impl_all!`]: https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_not_impl_all.html
[`assert_not_impl_any!`]: https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_not_impl_any.html
[`assert_obj_safe!`]:     https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_obj_safe.html
[`assert_type_eq_all!`]:  https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_type_eq_all.html
[`assert_type_ne_all!`]:  https://docs.rs/static_assertions/1.0.0/static_assertions/macro.assert_type_ne_all.html
[`const_assert!`]:        https://docs.rs/static_assertions/1.0.0/static_assertions/macro.const_assert.html
[`const_assert_eq!`]:     https://docs.rs/static_assertions/1.0.0/static_assertions/macro.const_assert_eq.html
[`const_assert_ne!`]:     https://docs.rs/static_assertions/1.0.0/static_assertions/macro.const_assert_ne.html
