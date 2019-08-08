/// Asserts that the type does **not** implement all of the given traits.
///
/// This can be used to ensure types implement auto traits such as [`Send`] and
/// [`Sync`], as well as traits with [blanket `impl`s][blanket].
///
/// Note that the combination of all provided traits is required to not be
/// implemented. If you want to check that none of multiple traits are
/// implemented you should invoke [`assert_not_impl_any`] instead.
///
/// # Examples
///
/// On stable Rust, using the macro requires a unique “label” when used in a
/// module scope:
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use] extern crate static_assertions;
/// # use static_assertions::_core::cell::Cell;
/// # fn main() {}
/// assert_not_impl_all!(ptr0; *const u16, Send, Sync);
/// assert_not_impl_all!(ptr1; *const u8, Send, Sync);
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
/// assert_not_impl_all!(&'static mut u8, Copy);
///
/// fn main() {
///     assert_not_impl_all!(u32, Into<usize>);
/// }
/// ```
///
/// The following example fails to compile since `u32` can be converted into
/// `u64`.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {}
/// assert_not_impl_all!(u32, Into<u64>);
/// ```
/// 
/// `Cell<u32>` is not both `Sync` and `Send`.
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use] extern crate static_assertions;
/// # use static_assertions::_core::cell::Cell;
/// # fn main() {}
/// assert_not_impl_all!(cell; Cell<u32>, Sync, Send);
/// ```
/// But it is `Send`, so this fails to compile.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # use static_assertions::_core::cell::Cell;
/// # fn main() {}
/// assert_not_impl_all!(cell; Cell<u32>, Send);
/// ```
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [`assert_not_impl_any`]: macro.assert_not_impl_any.html
/// [blanket]: https://doc.rust-lang.org/book/second-edition/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export(local_inner_macros)]
macro_rules! assert_not_impl_all {
    ($($xs:tt)+) => { _assert_not_impl_all!($($xs)+); };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export(local_inner_macros)]
macro_rules! _assert_not_impl_all {
    ($x:ty, $($t:path),+ $(,)*) => {
        const _: fn() -> () = || {
            #[allow(dead_code)]
            struct Invalid;
            trait AmbiguousIfImpl<A> { fn some_item() {} }

            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            impl<T: ?Sized $(+ $t)*> AmbiguousIfImpl<Invalid> for T {}

            let _ = <$x as AmbiguousIfImpl<_>>::some_item;
        };
    };
}

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export(local_inner_macros)]
macro_rules! _assert_not_impl_all {
    ($x:ty, $($t:path),+ $(,)*) => {
        {
            #[allow(dead_code)]
            struct Invalid;
            trait AmbiguousIfImpl<A> { fn some_item() {} }

            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            impl<T: ?Sized $(+ $t)*> AmbiguousIfImpl<Invalid> for T {}

            let _ = <$x as AmbiguousIfImpl<_>>::some_item;
        }
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_not_impl_all!($($xs)+); }
    };
}
