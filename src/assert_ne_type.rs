/// Asserts that types are _not_ equal.
///
/// # Examples
///
/// On stable Rust, using the macro requires a unique “label” when used in a
/// module scope:
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {}
/// assert_ne_type!(slices; [u8], [u16], str);
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
/// assert_ne_type!(u8, (u8,), [u8]);
/// ```
///
/// The following produces a compilation failure because `c_uchar` is a type
/// alias for `u8`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_ne_type!(std::os::raw::c_uchar, u8, u32);
/// assert_ne_type!(std::os::raw::c_uchar, u32, u8);
/// assert_ne_type!(u32, std::os::raw::c_uchar, u8);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! assert_ne_type {
    ($($xs:tt)+) => { _assert_ne_type!($($xs)+); };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export(local_inner_macros)]
macro_rules! _assert_ne_type {
    ($x:ty, $y:ty $(,)*) => {
        const _: fn() = || {
            trait MutuallyExclusive {}
            impl MutuallyExclusive for $x {}
            impl MutuallyExclusive for $y {}
        };
    };
    ($x:ty, $y:ty, $($z:ty),+ $(,)*) => {
        _assert_ne_type!($x, $($z),+);
        _assert_ne_type!($y, $($z),+);
    };
}

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export(local_inner_macros)]
macro_rules! _assert_ne_type {
    ($x:ty, $y:ty $(,)*) => {
        {
            trait MutuallyExclusive {}
            impl MutuallyExclusive for $x {}
            impl MutuallyExclusive for $y {}
        }
    };
    ($x:ty, $y:ty, $($z:ty),+ $(,)*) => {
        _assert_ne_type!($x, $($z),+);
        _assert_ne_type!($y, $($z),+);
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_ne_type!($($xs)+); }
    };
}
