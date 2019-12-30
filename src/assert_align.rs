/// Asserts that the types' alignments are equal.
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

/// Asserts that the types' alignments are equal.
///
/// This macro has been deprecated in favor of
/// [`assert_align_eq!`](macro.assert_align_eq.html).
#[deprecated(
    since = "1.2.0",
    note = "Please use the 'assert_align_eq' macro instead"
)]
#[macro_export(local_inner_macros)]
macro_rules! assert_eq_align {
    ($($t:tt)*) => {
        assert_align_eq!($($t)*);
    };
}

/// Asserts that the types' alignments are **not** equal.
///
/// # Examples
///
/// A `u8` does not have the same alignment as a pointer:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_ne!(u8, *const u8);
/// ```
///
/// The following example fails to compile because a `usize` has the same
/// alignment as a pointer:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_ne!(*const u8, usize);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! assert_align_ne {
    ($x:ty, $($y:ty),+ $(,)?) => {
        const _: fn() = || {
            use $crate::_core::mem::align_of;
            const_assert_ne!(align_of::<$x>() $(, align_of::<$y>())+);
        };
    };
}

/// Asserts that the types' alignments are less than each other.
///
/// # Examples
///
/// A `u8` has smaller alignment than `u16`, which has smaller alignment than
/// a pointer:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_lt!(u8, u16, *const u8);
/// ```
///
/// The following example fails to compile because a `usize` has the same
/// alignment as a pointer:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_lt!(*const u8, usize);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! assert_align_lt {
    ($x:ty, $($y:ty),+ $(,)?) => {
        const _: fn() = || {
            use $crate::_core::mem::align_of;
            const_assert_lt!(align_of::<$x>() $(, align_of::<$y>())+);
        };
    };
}

/// Asserts that the types' alignments are less than or equal to each other.
///
/// # Examples
///
/// A `u8` and `i8` have smaller alignment than any pointer type:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_le!(u8, i8, *const u8);
/// ```
///
/// The following example fails to compile because a `usize` has greater
/// alignment than `u8`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_le!(usize, u8);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! assert_align_le {
    ($x:ty, $($y:ty),+ $(,)?) => {
        const _: fn() = || {
            use $crate::_core::mem::align_of;
            const_assert_le!(align_of::<$x>() $(, align_of::<$y>())+);
        };
    };
}

/// Asserts that the types' alignments are greater than each other.
///
/// # Examples
///
/// A pointer has greater alignment than `u16`, which has greater alignment than
/// `u8`:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_gt!(*const u8, u16, u8);
/// ```
///
/// The following example fails to compile because a `usize` has the same
/// alignment as a pointer:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_gt!(*const u8, usize);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! assert_align_gt {
    ($x:ty, $($y:ty),+ $(,)?) => {
        const _: fn() = || {
            use $crate::_core::mem::align_of;
            const_assert_gt!(align_of::<$x>() $(, align_of::<$y>())+);
        };
    };
}

/// Asserts that the types' alignments are greater than or equal to each other.
///
/// # Examples
///
/// A pointer has greater alignment than `u8` and `i8`:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_ge!(*const u8, u8, i8);
/// ```
///
/// The following example fails to compile because a `u8` has smaller alignment
/// than `usize`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_align_ge!(u8, usize);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! assert_align_ge {
    ($x:ty, $($y:ty),+ $(,)?) => {
        const _: fn() = || {
            use $crate::_core::mem::align_of;
            const_assert_ge!(align_of::<$x>() $(, align_of::<$y>())+);
        };
    };
}
