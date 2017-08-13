//! Compile-time assertions to ensure that invariants are met.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! static_assertions = "0.1"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! #[macro_use]
//! extern crate static_assertions;
//! # fn main() {}
//! ```
//!
//! # Assert Equal Size
//!
//! When performing operations such as pointer casts or dealing with [`usize`]
//! versus [`u64`] versus [`u32`], the size of your types matter. This is where
//! [`assert_eq_size`] comes into play. Types provided as arguments to
//! [`assert_eq_size`] are ensured to be the same size at compile-time. If the
//! types differ in size, the code will fail to compile.
//!
//! ```
//! # #[macro_use]
//! # extern crate static_assertions;
//! # fn main() {
//! assert_eq_size!([u8; 4], (u16, u16), u32);
//!
//! // Produces a compilation failure:
//! // assert_eq_size!(u32, u8);
//! # }
//! ```
//!
//! Similar to [`assert_eq_size`], there is [`assert_eq_size_val`]. Instead of
//! specifying types to compare, values' sizes can be directly compared against
//! each other.
//!
//! ```
//! # #[macro_use]
//! # extern crate static_assertions;
//! # fn main() {
//! let x = 42u8;
//! let y = true;
//!
//! assert_eq_size_val!(x, y);
//! # }
//! ```
//! [`assert_eq_size_val`] doesn't consume its arguments and thus works for
//! non-[`Clone`]able values.
//!
//! ```
//! # #[macro_use]
//! # extern crate static_assertions;
//! # fn main() {
//! struct Buffer([u8; 256]);
//!
//! let buf = Buffer([0; 256]);
//! let val = [0u64; 32];
//!
//! assert_eq_size_val!(buf, val);
//!
//! // `buf` and `val` can be used here
//! # }
//! ```
//!
//! # Assert Constant Expression
//!
//! Constant expressions can be ensured to have certain properties via
//! [`const_assert`]. If the expression evaluates to `false`, the file will fail
//! to compile. This is synonymous to [`static_assert` in C++][static_assert].
//!
//! ```
//! # #[macro_use]
//! # extern crate static_assertions;
//! # fn main() {
//! const NUM: usize = 32;
//!
//! const_assert!(NUM * NUM == 1024);
//! # }
//! ```
//!
//! As a shorthand for `const_assert!(a == b)`, there's [`const_assert_eq`]:
//!
//! ```
//! # #[macro_use]
//! # extern crate static_assertions;
//! # fn main() {
//! const NUM: usize = 32;
//! const_assert_eq!(NUM + NUM, 64);
//!
//! const TWO: usize = 2;
//! const_assert_eq!(TWO * TWO, TWO + TWO, 4);
//! # }
//! ```
//!
//! # Limitations
//!
//! Due to implementation details, [`assert_eq_size`], [`const_assert`], and
//! [`const_assert_eq`] can only be used from within the context of a function.
//!
//! [crate]: https://crates.io/crates/static_assertions
//! [static_assert]: http://en.cppreference.com/w/cpp/language/static_assert
//! [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
//! [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
//! [`u64`]: https://doc.rust-lang.org/std/primitive.u64.html
//! [`u32`]: https://doc.rust-lang.org/std/primitive.u32.html
//! [`assert_eq_size_val`]: macro.assert_eq_size_val.html
//! [`assert_eq_size`]: macro.assert_eq_size.html
//! [`const_assert`]: macro.const_assert.html
//! [`const_assert_eq`]: macro.const_assert_eq.html

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
/// # #[macro_use]
/// # extern crate static_assertions;
/// struct Byte(u8);
///
/// # fn main() {
/// assert_eq_size!(Byte, u8);
///
/// // Supports unlimited arguments:
/// assert_eq_size!([Byte; 4], [u16; 2], u32);
///
/// // Fails to compile:
/// // assert_eq_size!(Byte, u16);
/// # }
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

/// Asserts at compile-time that the constants are equal in value.
#[macro_export]
macro_rules! const_assert_eq {
    ($x:expr, $($xs:expr),+) => {
        const_assert!($($x == $xs),+);
    }
}
