/// Asserts that types are equal.
///
/// # Examples
///
/// Often times, type aliases are used to express usage semantics via naming. In
/// some cases, the underlying type may differ based on platform. However, other
/// types like [`c_float`] will always alias the same type.
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// use std::os::raw::c_float;
///
/// assert_eq_type!(c_float, f32);
/// ```
///
/// This macro can also be used to compare types that involve lifetimes! Just
/// use `'static` in that case:
///
/// ```
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// type Buf<'a> = &'a [u8];
///
/// assert_eq_type!(Buf<'static>, &'static [u8]);
/// # }
/// ```
///
/// The following example fails to compile because `String` and `str` do not
/// refer to the same type:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_eq_type!(String, str);
/// ```
///
/// This should also work the other way around, regardless of [`Deref`]
/// implementations.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_eq_type!(str, String);
/// ```
///
/// [`c_float`]: https://doc.rust-lang.org/std/os/raw/type.c_float.html
/// [`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
#[macro_export]
macro_rules! assert_eq_type {
    ($x:ty, $($xs:ty),+ $(,)*) => {
        const _: fn() = || { $({
            trait TypeEq {
                type This: ?Sized;
            }
            impl<T: ?Sized> TypeEq for T {
                type This = Self;
            }
            fn assert_eq_type<T: ?Sized, U: ?Sized>() where T: TypeEq<This = U> {}
            assert_eq_type::<$x, $xs>();
        })+ };
    };
}

/// Asserts that types are _not_ equal.
///
/// # Examples
///
/// Rust has all sorts of slices, but they represent different types of data:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_ne_type!([u8], [u16], str);
/// ```
///
/// The following example fails to compile because [`c_uchar`] is a type alias
/// for [`u8`]:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// use std::os::raw::c_uchar;
///
/// assert_ne_type!(c_uchar, u8, u32);
/// ```
///
/// [`c_uchar`]: https://doc.rust-lang.org/std/os/raw/type.c_uchar.html
/// [`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
#[macro_export]
macro_rules! assert_ne_type {
    ($x:ty, $($y:ty),+ $(,)?) => {
        const _: fn() = || {
            trait MutuallyExclusive {}
            impl MutuallyExclusive for $x {}
            $(impl MutuallyExclusive for $y {})+
        };
    };
}
