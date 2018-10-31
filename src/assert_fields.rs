/// Asserts that the type has the given fields.
///
/// # Examples
///
/// This may be used when types have odd fields as a result of `#[cfg]`.
///
/// ```
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// struct Ty {
///     #[cfg(windows)]
///     value: u8,
///     #[cfg(not(windows))]
///     value: usize,
/// }
///
/// /* ... */
///
/// // Always have `value` regardless of OS
/// assert_fields!(Ty, value);
/// # }
/// ```
///
/// Range does not have a field named `middle`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// use std::ops::Range;
///
/// assert_fields!(Range<u32>, middle);
/// # }
/// ```
#[macro_export]
macro_rules! assert_fields {
    ($t:path, $($f:ident),+) => {
        #[allow(unknown_lints, unneeded_field_pattern)]
        { $(let $t { $f: _, .. };)+ }
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_fields!($($xs)+); }
    };
}
