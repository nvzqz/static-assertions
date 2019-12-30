//! [![Banner](https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Banner.png)](https://github.com/nvzqz/static-assertions-rs)
//!
//! <div align="center">
//!     <a href="https://crates.io/crates/proc_static_assertions">
//!         <img src="https://img.shields.io/crates/d/proc_static_assertions.svg" alt="Downloads">
//!     </a>
//!     <a href="https://travis-ci.org/nvzqz/static-assertions-rs">
//!         <img src="https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master" alt="Build Status">
//!     </a>
//!     <br><br>
//! </div>
//!
//! Procedural macro [compile-time] assertions as an extension of
//! [`static_assertions`].
//!
//! # Usage
//!
//! There's two main ways of using this crate: as a direct dependency or
//! indirect dependency (via [`static_assertions`]).
//!
//! ## Direct Dependency
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! proc_static_assertions = "0.0.0"
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```
//! #[macro_use]
//! extern crate proc_static_assertions;
//! # fn main() {}
//! ```
//!
//! ## Indirect Dependency
//!
//! Add the following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! static_assertions = { version = "1.1.0", features = ["proc"] }
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```ignore
//! #[macro_use]
//! extern crate static_assertions;
//! ```
//!
//! This will also import all macros in `proc_static_assertions`.
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
//! [`static_assertions`]: https://github.com/nvzqz/static-assertions-rs
//! [crate]: https://crates.io/crates/static_assertions
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
//! [compile-time]: https://en.wikipedia.org/wiki/Compile_time

#![doc(html_root_url = "https://docs.rs/proc_static_assertions/0.0.0")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Icon.png"
)]
#![deny(missing_docs)]

extern crate proc_macro;
use proc_macro::TokenStream;

/// Statically assert aspects of types, traits, and more.
///
/// This currently does nothing. Create an issue if you have ideas for what this
/// could do!
#[proc_macro_attribute]
pub fn assert(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}
