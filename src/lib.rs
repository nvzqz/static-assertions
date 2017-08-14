//! Compile-time assertions to ensure that invariants are met.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! static_assertions = "0.2.2"
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
//! // Can be declared outside of a function if labeled
//! assert_eq_size!(bytes; (u8, u8), u16);
//!
//! // Fails to compile (same label):
//! // assert_eq_size!(bytes; u8, u8);
//!
//! fn main() {
//!     assert_eq_size!([u8; 4], (u16, u16), u32);
//!
//!     // Produces a compilation failure:
//!     // assert_eq_size!(u32, u8);
//! }
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
//! Rather than dereference a pointer to achieve the same effect as
//! [`assert_eq_size_val`], there is also the option of [`assert_eq_size_ptr`].
//!
//! # Assert Constant Expression
//!
//! Constant expressions can be ensured to have certain properties via
//! [`const_assert`]. If the expression evaluates to `false`, the file will fail
//! to compile. This is synonymous to [`static_assert` in C++][static_assert].
//!
//! As a [limitation](#limitations), a unique label is required if the macro is
//! used outside of a function.
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
//! const TWO: usize = 2;
//! const_assert_eq!(two; TWO * TWO, TWO + TWO, 4);
//!
//! // Fails to compile (same label):
//! // const_assert_eq!(two; TWO, TWO);
//!
//! fn main() {
//!     const NUM: usize = 32;
//!     const_assert_eq!(NUM + NUM, 64);
//! }
//! ```
//!
//! # Assert Object Safety
//!
//! Sometimes changes are made to traits that prevent them from being used in
//! the context of an object. Such a case would be adding a generic method and
//! forgetting to add `where Self: Sized` after it. If left unnoticed, that
//! mistake will end up affecting crate users and break compatibility.
//!
//! [`assert_obj_safe`] is here to save you from those troubles:
//!
//! ```
//! # #[macro_use]
//! # extern crate static_assertions;
//! assert_obj_safe!(basic; Send, Sync, AsRef<str>);
//!
//! trait MySafeTrait {}
//!
//! trait MyUnsafeTrait {
//!     fn generic<T>();
//! }
//!
//! fn main() {
//!     assert_obj_safe!(MySafeTrait);
//!
//!     // Produces a compilation failure:
//!     // assert_obj_safe!(MyUnsafeTrait);
//! }
//! ```
//!
//! # Assert Trait `impl`
//!
//! To ensure types implement [`Send`], [`Sync`], and other traits, there's
//! [`assert_impl`]:
//!
//! ```
//! # #[macro_use]
//! # extern crate static_assertions;
//! assert_impl!(str; String, Send, Sync, From<&'static str>);
//! assert_impl!(vec; &'static [u8], Into<Vec<u8>>);
//!
//! fn main() {
//!     // Produces a compilation failure:
//!     // `*const u8` cannot be sent between threads safely
//!     // assert_impl!(*const u8, Send);
//! }
//! ```
//!
//! # Limitations
//!
//! Due to implementation details, the following can only be used normally from
//! within the context of a function:
//!
//! - [`assert_eq_size`]
//! - [`assert_obj_safe`]
//! - [`assert_impl`]
//! - [`const_assert`]
//! - [`const_assert_eq`]
//!
//! To use these macros in other contexts, a unique label must be provided.
//!
//! If you want to read up about this and provide feedback, see
//! [the related issue on GitHub][issue1].
//!
//! [issue1]: https://github.com/nvzqz/static-assertions-rs/issues/1
//! [crate]: https://crates.io/crates/static_assertions
//! [static_assert]: http://en.cppreference.com/w/cpp/language/static_assert
//! [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
//! [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
//! [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
//! [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
//! [`u64`]: https://doc.rust-lang.org/std/primitive.u64.html
//! [`u32`]: https://doc.rust-lang.org/std/primitive.u32.html
//! [`assert_eq_size_val`]: macro.assert_eq_size_val.html
//! [`assert_eq_size_ptr`]: macro.assert_eq_size_ptr.html
//! [`assert_eq_size`]: macro.assert_eq_size.html
//! [`assert_obj_safe`]: macro.assert_obj_safe.html
//! [`assert_impl`]: macro.assert_impl.html
//! [`const_assert`]: macro.const_assert.html
//! [`const_assert_eq`]: macro.const_assert_eq.html

#![no_std]

#[doc(hidden)]
pub extern crate core as _core;

/// Asserts at compile-time that the types have equal sizes.
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// struct Byte(u8);
///
/// assert_eq_size!(pair; (u16, u16), [u16; 2], [u8; 4]);
///
/// // Fails to compile (same label):
/// // assert_eq_size!(pair; u8, u8);
///
/// fn main() {
///     assert_eq_size!(Byte, u8);
///
///     // Supports unlimited arguments:
///     assert_eq_size!([Byte; 4], [u16; 2], u32);
///
///     // Produces a compilation failure:
///     // assert_eq_size!(Byte, u16);
///
///     // Can also be used to assert number of bytes:
///     assert_eq_size!(u64, 8);
///     assert_eq_size!(tuple; (u8, u8), 2);
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
    };
    ($x:ty, $size:expr) => {
        assert_eq_size!($x, [u8; $size]);
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_eq_size!($($xs)+); }
    };
}

/// Asserts at compile-time that the values pointed to have equal sizes.
///
/// This especially is useful for when coercing pointers between different types
/// and ensuring the underlying values are the same size.
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// fn operation(x: &(u32, u32), y: &[u16; 4]) {
///     assert_eq_size_ptr!(x, y);
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! assert_eq_size_ptr {
    ($x:expr, $($xs:expr),+) => {
        #[allow(unused_unsafe)]
        unsafe {
            use $crate::_core::{mem, ptr};
            let mut copy = ptr::read($x);
            $(ptr::write(&mut copy, mem::transmute(ptr::read($xs)));)+
            mem::forget(copy);
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
        assert_eq_size_ptr!(&$x, $(&$xs),+);
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
/// // Produces a compilation failure:
/// // const_assert!(1 >= 2);
/// # }
/// ```
#[macro_export]
macro_rules! const_assert {
    ($($xs:expr),+) => {
        let _ = [(); 0 - (!($($xs)&&+) as usize)];
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { const_assert!($($xs)+); }
    };
}

/// Asserts at compile-time that the constants are equal in value.
#[macro_export]
macro_rules! const_assert_eq {
    ($x:expr, $($xs:expr),+) => {
        const_assert!($($x == $xs),+);
    };
    ($label:ident; $x:expr, $($xs:expr),+) => {
        const_assert!($label; $($x == $xs),+);
    };
}

/// Asserts at compile-time that the traits are object-safe.
///
/// This is useful for when changes are made to a trait that accidentally
/// prevent it from being used as an object.
#[macro_export]
macro_rules! assert_obj_safe {
    ($($xs:ty),+) => {
        $(let _: Option<&$xs> = None;)+
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_obj_safe!($($xs)+); }
    };
}

/// Asserts at compile-time that the type implements the given traits.
#[macro_export]
macro_rules! assert_impl {
    (
        $x:ty,
        $(
            $y:ident
            $(< $($args:ty),+ $(,)* >)*
        ),+
    ) => {
        $({
            fn assert_impl<T: ?Sized + $y $(< $($args),+ >)* >() {}
            assert_impl::<$x>();
        })+
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_impl!($($xs)+); }
    };
}
