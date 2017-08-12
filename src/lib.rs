#![no_std]

#[doc(hidden)]
pub extern crate core as _core;

/// Asserts at compile-time that the types have equal sizes.
///
/// This especially is useful for when coercing pointers between different types
/// and ensuring the underlying values are the same size.
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
    ($x:ty, $($xs:ty),+) => {
        #[allow(unused_unsafe)]
        unsafe {
            use $crate::_core::mem::{forget, transmute, uninitialized};
            $(forget::<$xs>(transmute(uninitialized::<$x>()));)+
        }
    }
}

/// Asserts at compile-time that the values have equal sizes.
#[macro_export]
macro_rules! assert_eq_size_val {
    ($x:expr, $($xs:expr),+) => {
        #[allow(unused_unsafe)]
        unsafe {
            use $crate::_core::{mem, ptr};
            let mut copy = ptr::read(&$x);
            $(ptr::write(&mut copy, mem::transmute(ptr::read(&$xs)));)+
            mem::forget(copy);
        }
    }
}

/// Asserts at compile-time that the constant expression evaluates to `true`.
#[macro_export]
macro_rules! const_assert {
    ($cond:expr) => {
        // Causes overflow if condition is false
        let _ = [(); 0 - (!($cond) as usize)];
    };
    ($($xs:expr),+) => {
        const_assert!($($xs)&&+);
    };
    ($($xs:expr);+ $(;)*) => {
        const_assert!($($xs),+);
    };
}