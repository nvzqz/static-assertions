/// Asserts that constant expressions evaluate to `true`.
///
/// Constant expressions can be ensured to have certain properties via this
/// macro. If the expression evaluates to `false`, the module will fail to compile.
/// This is similar to [`static_assert` in C++][static_assert].
///
/// # Alternatives
///
/// There also exist [`const_assert_eq`](macro.const_assert_eq.html) for
/// validating whether a sequence of expressions are equal to one another.
///
/// # Examples
///
/// A common use case is to guarantee properties about a constant value that's
/// generated via meta-programming.
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// const VALUE: i32 = // ...
/// # 3;
///
/// const_assert!(VALUE >= 2);
/// ```
///
/// Inputs are type-checked as booleans:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// const_assert!(!0);
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
/// The following fails to compile because multiplying by 5 does not have an
/// identity property:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// const_assert!(5 * 5 == 5);
/// ```
///
/// [static_assert]: http://en.cppreference.com/w/cpp/language/static_assert
#[macro_export(local_inner_macros)]
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        const _: $crate::True = _to_bool!($x);
    };
}

/// Asserts that constants are equal in value.
///
/// Use [`const_assert_eq_usize!`](macro.const_assert_eq_usize.html) for better
/// error messages when asserting
/// [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) equality.
///
/// # Examples
///
/// This works as a shorthand for `const_assert!(a == b)`:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// const TWO: i32 = 2;
///
/// const_assert_eq!(TWO * TWO, TWO + TWO);
/// ```
///
/// Just because 2 × 2 = 2 + 2, doesn't mean it holds true for other numbers:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// const_assert_eq!(4 + 4, 4 * 4);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! const_assert_eq {
    ($x:expr, $($y:expr),+ $(,)?) => {
        const_assert!($($x == $y)&&+);
    };
}

/// Asserts that constants of type
/// [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) are equal in
/// value.
///
/// This is equivalent to [`const_assert_eq!`](macro.const_assert_eq.html) but
/// allows for inspecting the values in error messages.
#[macro_export]
macro_rules! const_assert_eq_usize {
    ($x:expr, $($y:expr),+ $(,)?) => {
        // Assigned instance must match the annotated type or else it will fail.
        $(const _: [(); $x] = [(); $y];)+
    };
}

/// Asserts that constants are **not** equal in value.
///
/// # Examples
///
/// This works as a shorthand for `const_assert!(a != b)`:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// const NUM: usize = 32;
///
/// const_assert_ne!(NUM * NUM, 64);
/// ```
///
/// The following example fails to compile because 2 is magic and 2 × 2 = 2 + 2:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// const_assert_ne!(2 + 2, 2 * 2);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! const_assert_ne {
    ($x:expr, $($y:expr),+ $(,)?) => {
        const_assert!($($x != $y)&&+);
    };
}

/// Asserts that constants are less than each other.
#[macro_export(local_inner_macros)]
macro_rules! const_assert_lt {
    ($x:expr, $($y:expr),+ $(,)?) => {
        const_assert_lt!(@build $x, $($y),+);
    };
    (@build $x:expr) => {};
    (@build $x:expr, $($y:expr),+) => {
        const_assert!($x < _head!($($y),+));
        const_assert_lt!(@build $($y),+);
    };
}

/// Asserts that constants are less than or equal to each other.
#[macro_export(local_inner_macros)]
macro_rules! const_assert_le {
    ($x:expr, $($y:expr),+ $(,)?) => {
        const_assert_le!(@build $x, $($y),+);
    };
    (@build $x:expr) => {};
    (@build $x:expr, $($y:expr),+) => {
        const_assert!($x <= _head!($($y),+));
        const_assert_le!(@build $($y),+);
    };
}

/// Asserts that constants are greater than each other.
#[macro_export(local_inner_macros)]
macro_rules! const_assert_gt {
    ($x:expr, $($y:expr),+ $(,)?) => {
        const_assert_gt!(@build $x, $($y),+);
    };
    (@build $x:expr) => {};
    (@build $x:expr, $($y:expr),+) => {
        const_assert!($x > _head!($($y),+));
        const_assert_gt!(@build $($y),+);
    };
}

/// Asserts that constants are greater than or equal to each other.
#[macro_export(local_inner_macros)]
macro_rules! const_assert_ge {
    ($x:expr, $($y:expr),+ $(,)?) => {
        const_assert_ge!(@build $x, $($y),+);
    };
    (@build $x:expr) => {};
    (@build $x:expr, $($y:expr),+) => {
        const_assert!($x >= _head!($($y),+));
        const_assert_ge!(@build $($y),+);
    };
}
