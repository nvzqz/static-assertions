//! Compile-time assertions to ensure that invariants are met.
//!
//! _All_ assertions within this crate are performed at **compile-time**. This
//! allows for finding errors quickly and early when it comes to ensuring
//! certain features or aspects of a codebase.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! static_assertions = "0.2.5"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! #[macro_use]
//! extern crate static_assertions;
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! Very thorough examples are provided in the docs for each individual macro.
//! Failure case examples are also documented.
//!
//! # Limitations
//!
//! Due to implementation details, some macros can only be used normally from
//! within the context of a function. To use these macros in other contexts, a
//! unique label must be provided.
//!
//! ```compile_fail
//! # #[macro_use] extern crate static_assertions;
//! # fn main() {}
//! // error: expected item after attributes
//! const_assert!(true == true);
//! ```
//!
//! This can be fixed via:
//!
//! ```
//! # #[macro_use] extern crate static_assertions;
//! # fn main() {}
//! const_assert!(label; true == true);
//! ```
//!
//! This issue can be followed [here][issue1]. Feedback and potential solutions
//! are welcome!
//!
//! [issue1]: https://github.com/nvzqz/static-assertions-rs/issues/1
//! [crate]: https://crates.io/crates/static_assertions

#![no_std]

#![deny(unused_macros)]

#[doc(hidden)]
pub extern crate core as _core;

mod assert_cfg;
mod assert_eq_size;
mod assert_fields;
mod assert_impl;
mod assert_obj_safe;
mod const_assert;
