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
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// # fn main() {
/// struct Byte(u8);
///
/// let x = 10u8;
/// let y = Byte(42); // Works for non-cloneable types
///
/// assert_eq_size_val!(x, y);
/// assert_eq_size_val!(x, y, 0u8);
///
/// // Fails to compile:
/// // assert_eq_size_val!(x, 0u32);
/// # }
/// ```
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
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// # fn main() {
/// const_assert!(2 + 2 == 4);
///
/// const FIVE: usize = 5;
/// const_assert!(FIVE - FIVE == 0);
///
/// // Fails to compile:
/// // const_assert!(1 >= 2);
/// # }
/// ```
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
