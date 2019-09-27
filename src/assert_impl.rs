/// Asserts that the type implements _all_ of the given traits.
///
/// See [`assert_not_impl_all!`] for achieving the opposite effect.
///
/// # Examples
///
/// This can be used to ensure types implement auto traits such as [`Send`] and
/// [`Sync`], as well as traits with [blanket `impl`s][blanket].
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_all!(u32: Copy, Send);
/// assert_impl_all!(&str: Into<String>);
/// ```
///
/// The following example fails to compile because raw pointers do not implement
/// [`Send`] since they cannot be moved between threads safely:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_all!(*const u8: Send);
/// ```
///
/// [`assert_not_impl_all!`]: macro.assert_not_impl_all.html
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [blanket]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export]
macro_rules! assert_impl_all {
    ($type:ty: $($trait:path),+ $(,)?) => {
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized $(+ $trait)+>() {}
            assert_impl_all::<$type>();
        };
    };
}

/// Asserts that the type does **not** implement _all_ of the given traits.
///
/// This can be used to ensure types do not implement auto traits such as
/// [`Send`] and [`Sync`], as well as traits with [blanket `impl`s][blanket].
///
/// Note that the combination of all provided traits is required to not be
/// implemented. If you want to check that none of multiple traits are
/// implemented you should invoke [`assert_not_impl_any!`] instead.
///
/// # Examples
///
/// Although `u32` implements `From<u16>`, it does not implement `Into<usize>`:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_not_impl_all!(u32: From<u16>, Into<usize>);
/// ```
///
/// The following example fails to compile since `u32` can be converted into
/// `u64`.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_not_impl_all!(u32: Into<u64>);
/// ```
///
/// The following compiles because [`Cell`] is not both [`Sync`] _and_ [`Send`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// use std::cell::Cell;
///
/// assert_not_impl_all!(Cell<u32>: Sync, Send);
/// ```
///
/// But it is [`Send`], so this fails to compile:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// # std::cell::Cell;
/// assert_not_impl_all!(Cell<u32>: Send);
/// ```
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [`assert_not_impl_any!`]: macro.assert_not_impl_any.html
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [blanket]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export]
macro_rules! assert_not_impl_all {
    ($x:ty: $($t:path),+ $(,)?) => {
        const _: fn() = || {
            #[allow(dead_code)]
            struct Invalid;
            trait AmbiguousIfImpl<A> { fn some_item() {} }

            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            impl<T: ?Sized $(+ $t)+> AmbiguousIfImpl<Invalid> for T {}

            let _ = <$x as AmbiguousIfImpl<_>>::some_item;
        };
    };
}

/// Asserts that the type does **not** implement _any_ of the given traits.
///
/// This can be used to ensure types do not implement auto traits such as
/// [`Send`] and [`Sync`], as well as traits with [blanket `impl`s][blanket].
///
/// This macro causes a compilation failure if any of the provided individual
/// traits are implemented for the type. If you want to check that a combination
/// of traits is not implemented you should invoke [`assert_not_impl_all!`]
/// instead. For single traits both macros behave the same.
///
/// # Examples
///
/// If `u32` were to implement `Into` conversions for `usize` _and_ for `u8`,
/// the following would fail to compile:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_not_impl_any!(u32: Into<usize>, Into<u8>);
/// ```
///
/// This is also good for simple one-off cases:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_not_impl_any!(&'static mut u8: Copy);
/// ```
///
/// The following example fails to compile since `u32` can be converted into
/// `u64` even though it can not be converted into a `u16`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_not_impl_any!(u32: Into<u64>, Into<u16>);
/// ```
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [`assert_not_impl_all!`]: macro.assert_not_impl_all.html
/// [blanket]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export]
macro_rules! assert_not_impl_any {
    ($x:ty: $($t:path),+ $(,)?) => {
        const _: fn() = || {
            trait AmbiguousIfImpl<A> { fn some_item() {} }

            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            $({
                #[allow(dead_code)]
                struct Invalid;
                impl<T: ?Sized + $t> AmbiguousIfImpl<Invalid> for T {}
            })+

            let _ = <$x as AmbiguousIfImpl<_>>::some_item;
        };
    };
}
