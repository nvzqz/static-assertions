#![no_std]

#[doc(hidden)]
pub extern crate core as _core;

/// Asserts at compile-time that the two types have equal sizes.
#[macro_export]
macro_rules! assert_eq_size {
    ($x:ty, $y:ty) => {
        #[allow(unused_unsafe)]
        unsafe {
            use $crate::_core::mem::{forget, transmute, uninitialized};
            forget::<$y>(transmute(uninitialized::<$x>()));
        }
    }
}
