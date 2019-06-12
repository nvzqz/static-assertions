/// Asserts that types are equal.
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
/// type A = u8;
/// type B = A;
///
/// assert_eq_type!(byte; u8, A, B);
/// ```
///
/// The [labeling limitation](index.html#limitations) is not necessary if
/// compiling on nightly Rust with the `nightly` feature enabled:
///
#[cfg_attr(feature = "nightly", doc = "```")]
#[cfg_attr(not(feature = "nightly"), doc = "```ignore")]
/// #![feature(underscore_const_names)]
/// # #[macro_use] extern crate static_assertions;
/// # type A = u8;
/// # type B = A;
///
/// assert_eq_type!(u8, A, B);
/// ```
///
/// The following produces a compilation failure because `String` and `str` do
/// not refer to the same type:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_eq_type!(String, str);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! assert_eq_type {
    ($($xs:tt)+) => { _assert_eq_type!($($xs)+); };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export(local_inner_macros)]
macro_rules! _assert_eq_type {
    ($x:ty, $($xs:ty),+ $(,)*) => {
        const _: fn() = || { $( {
            struct Type<A: ?Sized>(A);
            fn assert_eq_type(x: &Type<$x>) -> &Type<$xs> { x }
        } )+ };
    };
}

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export(local_inner_macros)]
macro_rules! _assert_eq_type {
    ($x:ty, $($xs:ty),+ $(,)*) => { $( {
        struct Type<A: ?Sized>(A);
        fn assert_eq_type(x: &Type<$x>) -> &Type<$xs> { x }
    } )+ };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_eq_type!($($xs)+); }
    };
}
