/// Asserts that the type has the given fields.
///
/// # Examples
///
/// One common use case is when types have fields defined multiple times as a
/// result of `#[cfg]`. This can be an issue when exposing a public API.
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use] extern crate static_assertions;
/// pub struct Ty {
///     #[cfg(windows)]
///     pub val1: u8,
///     #[cfg(not(windows))]
///     pub val1: usize,
///
///     #[cfg(unix)]
///     pub val2: u32,
///     #[cfg(not(unix))]
///     pub val2: usize,
/// }
///
/// // Requires a unique label in module scope
/// assert_fields!(windows; Ty, val1);
///
/// fn main() {
///     // Always have `val2` regardless of OS
///     assert_fields!(Ty, val2);
/// }
/// ```
///
/// The [labeling limitation](index.html#limitations) is not necessary if
/// compiling on nightly Rust with the `nightly` feature enabled:
///
#[cfg_attr(feature = "nightly", doc = "```")]
#[cfg_attr(not(feature = "nightly"), doc = "```ignore")]
/// #![feature(underscore_const_names)]
/// # #[macro_use] extern crate static_assertions;
///
/// use std::ops::Range;
///
/// assert_fields!(Range<u32>, start, end);
/// ```
///
/// Range does not have a field named `middle`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// # use std::ops::Range;
/// assert_fields!(Range<u32>, middle);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! assert_fields {
    ($($xs:tt)+) => { _assert_fields!($($xs)+); };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export(local_inner_macros)]
macro_rules! _assert_fields {
    ($t:path, $($f:ident),+) => {
        #[allow(unknown_lints, unneeded_field_pattern)]
        const _: fn() -> () = || {
            $(let $t { $f: _, .. };)+
        };
    };
}

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export(local_inner_macros)]
macro_rules! _assert_fields {
    ($t:path, $($f:ident),+) => {
        #[allow(unknown_lints, unneeded_field_pattern)]
        { $(let $t { $f: _, .. };)+ }
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_fields!($($xs)+); }
    };
}