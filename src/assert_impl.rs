/// Asserts that the type implements _all_ of the given traits.
///
/// This is an alias for [`assert_impl_all!`](macro.assert_impl_all.html).
#[deprecated(since = "0.3.4", note = "please use `assert_impl_all!` instead")]
#[macro_export(local_inner_macros)]
macro_rules! assert_impl {
    ($($xs:tt)+) => { _assert_impl_all!($($xs)+); };
}

/// Asserts that the type implements _all_ of the given traits.
///
/// This can be used to ensure types implement auto traits such as [`Send`] and
/// [`Sync`], as well as traits with [blanket `impl`s][blanket].
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
/// assert_impl_all!(str; String, Send, Sync, From<&'static str>);
/// assert_impl_all!(vec; &'static [u8], Into<Vec<u8>>);
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
/// assert_impl_all!(u32, Copy, Send);
///
/// fn main() {
///     assert_impl_all!(&str, Into<String>);
/// }
/// ```
///
/// Raw pointers cannot be sent between threads safely:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_impl_all!(*const u8, Send);
/// # }
/// ```
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [blanket]: https://doc.rust-lang.org/book/second-edition/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export(local_inner_macros)]
macro_rules! assert_impl_all {
    ($($xs:tt)+) => { _assert_impl_all!($($xs)+); };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export(local_inner_macros)]
macro_rules! _assert_impl_all {
    ($x:ty, $($t:path),+ $(,)*) => {
        const _: fn() -> () = || {
            fn assert_impl_all<T>() where T: ?Sized $(+ $t)+ {}
            assert_impl_all::<$x>();
        };
    };
}

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export(local_inner_macros)]
macro_rules! _assert_impl_all {
    ($x:ty, $($t:path),+ $(,)*) => {
        {
            fn assert_impl_all<T>() where T: ?Sized $(+ $t)+ {}
            assert_impl_all::<$x>();
        }
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_impl_all!($($xs)+); }
    };
}
