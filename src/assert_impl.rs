/// Asserts that the type implements the given traits.
///
/// # Examples
///
/// Can be used to ensure types implement [`Send`], [`Sync`], and other traits:
///
/// ```
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {}
/// assert_impl!(str; String, Send, Sync, From<&'static str>);
/// assert_impl!(vec; &'static [u8], Into<Vec<u8>>);
/// ```
///
/// Raw pointers cannot be sent between threads safely:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_impl!(*const u8, Send);
/// # }
/// ```
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
#[macro_export]
macro_rules! assert_impl {
    ($x:ty, $($t:path),+ $(,)*) => {
        {
            fn assert_impl<T>() where T: ?Sized $(+ $t)+ {}
            assert_impl::<$x>();
        }
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_impl!($($xs)+); }
    };
}
