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
//! # Limitations
//!
//! Due to implementation details, some macros can only be used normally from
//! within the context of a function. To use these macros in other contexts, a
//! unique label must be provided.
//!
#![cfg_attr(feature = "nightly", doc = "```ignore")]
#![cfg_attr(not(feature = "nightly"), doc = "```compile_fail")]
//! # #[macro_use] extern crate static_assertions;
//! # fn main() {}
//! // error: expected item after attributes
//! const_assert!(true == true);
//! ```
//!
//! This can be fixed via:
//!
#![cfg_attr(feature = "nightly", doc = "```ignore")]
#![cfg_attr(not(feature = "nightly"), doc = "```")]
//! # #[macro_use] extern crate static_assertions;
//! # fn main() {}
//! const_assert!(label; true == true);
//! ```
//!
//! This can be followed at [issue #1][issue1].
//!
//! ## Labeling Limitation Fix
//!
//!  The labeling workaround is **not
//! necessary** (and is <span style="color:red">r<strong>emoved</strong></span>)
//! when compiling on nightly Rust with the `nightly` feature flag enabled. This
//! can be done by having the following in your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies.static_assertions]
//! version  = "0.3.4"
//! features = ["nightly"]
//! ```
//!
//! To compile with nightly Rust, run the following in your
//! [shell](https://en.wikipedia.org/wiki/Shell_(computing)) or
//! [command prompt](https://en.wikipedia.org/wiki/Command_Prompt) of choice:
//!
//! ```sh
//! rustup install nightly
//! cargo +nightly build
//! ```
//!
//! Notice that this also requires enabling the
//! [`underscore_const_names`](https://github.com/rust-lang/rust/issues/54912)
//! nightly Rust feature:
//!
#![cfg_attr(feature = "nightly", doc = "```")]
#![cfg_attr(not(feature = "nightly"), doc = "```ignore")]
//! #![feature(underscore_const_names)]
//! # #[macro_use] extern crate static_assertions;
//!
//! const_assert!(true != false);
//!
//! fn main() {
//!     const_assert!(false != true);
//! }
//! ```
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
mod assert_eq_type;
mod assert_fields;
mod assert_impl;
mod assert_not_impl;
mod assert_ne_type;
mod assert_obj_safe;
mod const_assert;
