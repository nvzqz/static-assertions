/// Asserts that constant expressions evaluate to `true`.
///
/// Constant expressions can be ensured to have certain properties via this
/// macro If the expression evaluates to `false`, the file will fail to compile.
/// This is synonymous to [`static_assert` in C++][static_assert].
///
/// # Alternatives
///
/// There also exists [`const_assert_eq`](macro.const_assert_eq.html) for
/// validating whether a sequence of expressions are equal to one another.
///
/// # Examples
///
/// Some expressions are blatantly false:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// const_assert!(1 >= 2);
/// # }
/// ```
///
/// Inputs are type-checked as booleans:
///
/// ```compile_fail
#[cfg_attr(feature = "nightly", doc = "#![feature(underscore_const_names)]")]
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// const_assert!(!0);
/// # }
/// ```
///
/// Despite this being a macro, we see this produces a type error:
///
/// ```txt
///   | const_assert!(!0);
///   |               ^^ expected bool, found integral variable
///   |
///   = note: expected type `bool`
///              found type `{integer}`
/// ```
///
/// On stable Rust, using the macro requires a unique “label” when used in a
/// module scope:
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use]
/// # extern crate static_assertions;
/// # fn main() {}
/// const_assert!(meaning_of_life; 42 == !!42);
/// ```
///
/// The [labeling limitation](index.html#limitations) is not necessary if
/// compiling on nightly Rust with the `nightly` feature enabled:
///
#[cfg_attr(feature = "nightly", doc = "```")]
#[cfg_attr(not(feature = "nightly"), doc = "```ignore")]
/// #![feature(underscore_const_names)]
/// # #[macro_use] extern crate static_assertions;
///
/// const FIVE: usize = 5;
///
/// const_assert!(FIVE * FIVE == 25);
///
/// fn main() {
///     const_assert!(2 + 2 == 4);
///     const_assert!(FIVE - FIVE == 0);
/// }
/// ```
///
/// [static_assert]: http://en.cppreference.com/w/cpp/language/static_assert
#[macro_export(local_inner_macros)]
macro_rules! const_assert {
    ($($xs:tt)+) => { _const_assert!($($xs)+); };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export(local_inner_macros)]
#[allow(dead_code)]
macro_rules! _const_assert {
    ($($xs:expr),+ $(,)*) => {
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !($({ const B: bool = $xs; B })&&+) as usize] = [];
    };
}

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export(local_inner_macros)]
macro_rules! _const_assert {
    ($($xs:expr),+ $(,)*) => {
        #[allow(unknown_lints, eq_op)]
        let _ = [(); 0 - !($({ const B: bool = $xs; B })&&+) as usize];
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { const_assert!($($xs)+); }
    };
}

/// Asserts that constants are equal in value.
///
/// # Examples
///
/// Works as a shorthand for `const_assert!(a == b)`:
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use]
/// # extern crate static_assertions;
/// const TWO: usize = 2;
/// const_assert_eq!(two; TWO * TWO, TWO + TWO, 4);
///
/// fn main() {
///     const NUM: usize = 32;
///     const_assert_eq!(NUM + NUM, 64);
/// }
/// ```
///
/// Just because 2 × 2 = 2 + 2 doesn't mean it holds true for other numbers:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// const_assert_eq!(4 + 4, 4 * 4);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! const_assert_eq {
    ($x:expr, $($xs:expr),+ $(,)*) => {
        const_assert!($($x == $xs),+);
    };
    ($label:ident; $x:expr, $($xs:expr),+ $(,)*) => {
        const_assert!($label; $($x == $xs),+);
    };
}

/// Asserts that constants are _not_ equal in value.
///
/// # Examples
///
/// Works as a shorthand for `const_assert!(a != b)`:
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use]
/// # extern crate static_assertions;
/// const_assert_ne!(nums; 1, 2, 3, 4);
///
/// fn main() {
///     const NUM: usize = 32;
///     const_assert_ne!(NUM * NUM, 64);
/// }
/// ```
///
/// The magic number 2, where 2 × 2 = 2 + 2:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// const_assert_ne!(2 + 2, 2 * 2);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! const_assert_ne {
    ($x:expr, $($xs:expr),+ $(,)*) => {
        const_assert!($($x != $xs),+);
    };
    ($label:ident; $x:expr, $($xs:expr),+ $(,)*) => {
        const_assert!($label; $($x != $xs),+);
    };
}
