/// Asserts that constant expressions evaluate to `true`.
///
/// There also exists [`const_assert_eq`](macro.const_assert_eq.html) for
/// validating whether a sequence of expressions are equal to one another.
///
/// # Examples
///
/// Constant expressions can be ensured to have certain properties via this
/// macro If the expression evaluates to `false`, the file will fail to compile.
/// This is synonymous to [`static_assert` in C++][static_assert].
///
/// As a [limitation](index.html#limitations), a unique label is required if
/// the macro is used outside of a function.
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// const FIVE: usize = 5;
///
/// const_assert!(twenty_five; FIVE * FIVE == 25);
///
/// fn main() {
///     const_assert!(2 + 2 == 4);
///     const_assert!(FIVE - FIVE == 0);
/// }
/// ```
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
/// [static_assert]: http://en.cppreference.com/w/cpp/language/static_assert
#[macro_export]
macro_rules! const_assert {
    ($($xs:expr),+ $(,)*) => {
        #[allow(unknown_lints, eq_op)]
        let _ = [(); 0 - !($($xs)&&+) as usize];
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
/// ```
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
#[macro_export]
macro_rules! const_assert_eq {
    ($x:expr, $($xs:expr),+ $(,)*) => {
        const_assert!($($x == $xs),+);
    };
    ($label:ident; $x:expr, $($xs:expr),+ $(,)*) => {
        const_assert!($label; $($x == $xs),+);
    };
}
