#![no_std]

#[doc(hidden)]
pub extern crate core as _core;

/// Asserts at compile-time that the types have equal sizes.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate static_assertions;
///
/// struct Byte(u8);
///
/// fn main() {
///     assert_eq_size!(Byte, u8);
///     // assert_eq_size!(Byte, u16); // Fails to compile
///
///     // Supports unlimited arguments:
///     assert_eq_size!([Byte; 4], [u16; 2], u32);
/// }
/// ```
#[macro_export]
macro_rules! assert_eq_size {
    ($x:ty, $y:ty) => {
        #[allow(unused_unsafe)]
        unsafe {
            use $crate::_core::mem::{forget, transmute, uninitialized};
            forget::<$y>(transmute(uninitialized::<$x>()));
        }
    };
    ($x:ty, $y:ty, $($rest:ty),+) => {
        assert_eq_size!($x, $y);
        assert_eq_size!($y, $($rest),+);
    };
}

/// Asserts at compile-time that the values have equal sizes.
#[macro_export]
macro_rules! assert_eq_size_val {
    ($x:expr, $y:expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            use $crate::_core::{mem, ptr};
            let (x, y) = (&$x, &$y);
            let mut copy = ptr::read(x);
            ptr::write(&mut copy, mem::transmute(ptr::read(y)));
            mem::forget(copy);
        }
    };
    ($x:expr, $y:expr, $($rest:expr),+) => {
        assert_eq_size_val!($x, $y);
        assert_eq_size_val!($y, $($rest),+);
    };
}
