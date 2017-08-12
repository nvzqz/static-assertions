#![no_std]

#[doc(hidden)]
pub extern crate core as _core;

/// Asserts at compile-time that the two types have equal sizes.
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
