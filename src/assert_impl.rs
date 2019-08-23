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
/// Raw pointers do not implement [`Send`] nor [`Sync`] because they cannot be
/// sent between threads safely:
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
        const _: fn() -> () = || {
            fn assert_impl_all<T: ?Sized $(+ $trait)+>() {}
            assert_impl_all::<$type>();
        };
    };
}
