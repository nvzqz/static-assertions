/// Asserts that types are equal in alignment.
///
/// This is useful when ensuring that pointer arithmetic is done correctly, or
/// when [FFI] requires a type to have the same alignment as some foreign type.
///
/// # Examples
///
/// A `usize` has the same alignment as any pointer type:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_eq!(usize, *const u8, *mut u8);
/// ```
///
/// The following passes because `[i32; 4]` has the same alignment as `i32`:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_eq!([i32; 4], i32);
/// ```
///
/// The following example fails to compile because `i32x4` explicitly has 4
/// times the alignment as `[i32; 4]`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// # #[allow(non_camel_case_types)]
/// #[repr(align(16))]
/// struct i32x4([i32; 4]);
///
/// assert_align_eq!(i32x4, [i32; 4]);
/// ```
///
/// [FFI]: https://en.wikipedia.org/wiki/Foreign_function_interface
#[macro_export(local_inner_macros)]
macro_rules! assert_align_eq {
    ($x:ty, $($y:ty),+ $(,)?) => {
        const _: fn() = || {
            use $crate::_core::mem::align_of;
            const_assert_eq_usize!(align_of::<$x>() $(, align_of::<$y>())+);
        };
    };
}

/// Asserts that types are equal in alignment.
///
/// This macro has been deprecated in favor of
/// [`assert_align_eq!`](macro.assert_align_eq.html).
#[deprecated(
    since = "1.2.0",
    note = "Please use the 'assert_align_eq' macro instead",
)]
#[macro_export(local_inner_macros)]
macro_rules! assert_eq_align {
    ($($t:tt)*) => {
        assert_align_eq!($($t)*);
    };
}
