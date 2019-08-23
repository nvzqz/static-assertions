//! [![Banner](https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Banner.png)](https://github.com/nvzqz/static-assertions-rs)
//!
//! Compile-time assertions to ensure that invariants are met.
//!
//! _All_ assertions within this crate are performed at [compile-time]. This
//! allows for finding errors quickly and early when it comes to ensuring
//! certain features or aspects of a codebase. These macros are especially
//! important when exposing a public API that requires types to be the same size
//! or implement certain traits.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! static_assertions = "0.3.4"
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```
//! #[macro_use]
//! extern crate static_assertions;
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! Very thorough examples are provided in the docs for
//! [each individual macro](#macros). Failure case examples are also documented.
//!
//! # Changes
//!
//! See [`CHANGELOG.md`](https://github.com/nvzqz/static-assertions-rs/blob/master/CHANGELOG.md)
//! for a complete list of what has changed from one version to another.
//!
//! # Donate
//!
//! This project is made freely available (as in free beer), but unfortunately
//! not all beer is free! So, if you would like to buy me a beer (or coffee or
//! *more*), then consider supporting my work that's benefited your project
//! and thousands of others.
//!
//! <a href="https://www.patreon.com/nvzqz">
//!     <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
//! </a>
//! <a href="https://www.paypal.me/nvzqz">
//!     <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
//! </a>
//!
//! [issue1]: https://github.com/nvzqz/static-assertions-rs/issues/1
//! [crate]: https://crates.io/crates/static_assertions
//! [compile-time]: https://en.wikipedia.org/wiki/Compile_time
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html

#![doc(html_root_url = "https://docs.rs/static_assertions/0.3.4")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Icon.png")]

#![no_std]

#![deny(unused_macros)]

#[doc(hidden)]
pub extern crate core as _core;

mod assert_cfg;
mod assert_eq_size;
mod assert_fields;
mod assert_impl;
mod assert_not_impl;
mod assert_obj_safe;
mod assert_type;
mod const_assert;
