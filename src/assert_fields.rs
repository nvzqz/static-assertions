/// Asserts that the type has the given fields.
///
/// # Examples
///
/// One common use case is when types have fields defined multiple times as a
/// result of `#[cfg]`. This can be an issue when exposing a public API.
///
/// ```
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
/// // Always have `val2` regardless of OS
/// assert_fields!(Ty, val2);
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
#[macro_export]
macro_rules! assert_fields {
    ($t:path, $($f:ident),+) => {
        #[allow(unknown_lints, unneeded_field_pattern)]
        const _: fn() -> () = || {
            $(let $t { $f: _, .. };)+
        };
    };
}
