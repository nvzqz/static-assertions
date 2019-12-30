/// Asserts that the type implements exactly one in a set of traits.
///
/// Related:
/// - [`assert_impl_any!`]
/// - [`assert_impl_all!`]
/// - [`assert_impl_not_all!`]
/// - [`assert_impl_not_any!`]
///
/// # Examples
///
/// Given some type `Foo`, it is expected to implement either `Snap`, `Crackle`,
/// or `Pop`:
///
/// ```compile_fail
/// # use static_assertions::assert_impl_one; fn main() {}
/// struct Foo;
///
/// trait Snap {}
/// trait Crackle {}
/// trait Pop {}
///
/// assert_impl_one!(Foo: Snap, Crackle, Pop);
/// ```
///
/// If _only_ `Crackle` is implemented, the assertion passes:
///
/// ```
/// # use static_assertions::assert_impl_one; fn main() {}
/// # struct Foo;
/// # trait Snap {}
/// # trait Crackle {}
/// # trait Pop {}
/// impl Crackle for Foo {}
///
/// assert_impl_one!(Foo: Snap, Crackle, Pop);
/// ```
///
/// If `Snap` or `Pop` is _also_ implemented, the assertion fails:
///
/// ```compile_fail
/// # use static_assertions::assert_impl_one; fn main() {}
/// # struct Foo;
/// # trait Snap {}
/// # trait Crackle {}
/// # trait Pop {}
/// # impl Crackle for Foo {}
/// impl Pop for Foo {}
///
/// assert_impl_one!(Foo: Snap, Crackle, Pop);
/// ```
///
/// [`assert_impl_any!`]:     macro.assert_impl_any.html
/// [`assert_impl_all!`]:     macro.assert_impl_all.html
/// [`assert_impl_not_all!`]: macro.assert_not_impl_all.html
/// [`assert_impl_not_any!`]: macro.assert_not_impl_any.html
#[macro_export]
macro_rules! assert_impl_one {
    ($x:ty: $($t:path),+ $(,)?) => {
        const _: fn() = || {
            // Generic trait that must be implemented for `$x` exactly once.
            trait AmbiguousIfMoreThanOne<A> {
                // Required for actually being able to reference the trait.
                fn some_item() {}
            }

            // Creates multiple scoped `Token` types for each trait `$t`, over
            // which a specialized `AmbiguousIfMoreThanOne<Token>` is
            // implemented for every type that implements `$t`.
            $({
                #[allow(dead_code)]
                struct Token;

                impl<T: ?Sized + $t> AmbiguousIfMoreThanOne<Token> for T {}
            })+

            // If there is only one specialized trait impl, type inference with
            // `_` can be resolved and this can compile. Fails to compile if
            // `$x` implements more than one `AmbiguousIfMoreThanOne<Token>` or
            // does not implement any at all.
            let _ = <$x as AmbiguousIfMoreThanOne<_>>::some_item;
        };
    };
}

/// Asserts that the type implements _all_ of the given traits.
///
/// See [`assert_impl_not_all!`] for achieving the opposite effect.
///
/// # Examples
///
/// This can be used to ensure types implement auto traits such as [`Send`] and
/// [`Sync`], as well as traits with [blanket `impl`s][blanket].
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_all!(u32: Copy, Send);
/// assert_impl_all!(&str: Into<String>);
/// ```
///
/// The following example fails to compile because raw pointers do not implement
/// [`Send`] since they cannot be moved between threads safely:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_all!(*const u8: Send);
/// ```
///
/// [`assert_impl_not_all!`]: macro.assert_not_impl_all.html
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [blanket]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export(local_inner_macros)]
macro_rules! assert_impl_all {
    ($ty:ty: $($traits:path),+ $(,)?) => {
        assert_impl!($ty: $( ($traits) )&+);
    };
}

/// Asserts that the type implements _any_ of the given traits.
///
/// See [`assert_impl_not_any!`] for achieving the opposite effect.
///
/// # Examples
///
/// `u8` cannot be converted from `u16`, but it can be converted into `u16`:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_any!(u8: From<u16>, Into<u16>);
/// ```
///
/// The unit type cannot be converted from `u8` or `u16`, but it does implement
/// [`Send`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_any!((): From<u8>, From<u16>, Send);
/// ```
///
/// The following example fails to compile because raw pointers do not implement
/// [`Send`] or [`Sync`] since they cannot be moved or shared between threads
/// safely:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_any!(*const u8: Send, Sync);
/// ```
///
/// [`assert_impl_not_any!`]: macro.assert_not_impl_any.html
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
#[macro_export(local_inner_macros)]
macro_rules! assert_impl_any {
    ($ty:ty: $($traits:path),+ $(,)?) => {
        assert_impl!($ty: $( ($traits) )|+);
    };
}

/// Asserts that the type does **not** implement _all_ of the given traits.
///
/// This can be used to ensure types do not implement auto traits such as
/// [`Send`] and [`Sync`], as well as traits with [blanket `impl`s][blanket].
///
/// Note that the combination of all provided traits is required to not be
/// implemented. If you want to check that none of multiple traits are
/// implemented you should invoke [`assert_impl_not_any!`] instead.
///
/// # Examples
///
/// Although `u32` implements `From<u16>`, it does not implement `Into<usize>`:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_not_all!(u32: From<u16>, Into<usize>);
/// ```
///
/// The following example fails to compile since `u32` can be converted into
/// `u64`.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_not_all!(u32: Into<u64>);
/// ```
///
/// The following compiles because [`Cell`] is not both [`Sync`] _and_ [`Send`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// use std::cell::Cell;
///
/// assert_impl_not_all!(Cell<u32>: Sync, Send);
/// ```
///
/// But it is [`Send`], so this fails to compile:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// # std::cell::Cell;
/// assert_impl_not_all!(Cell<u32>: Send);
/// ```
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [`assert_impl_not_any!`]: macro.assert_impl_not_any.html
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [blanket]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export(local_inner_macros)]
macro_rules! assert_impl_not_all {
    ($ty:ty: $($traits:path),+ $(,)?) => {
        assert_impl!($ty: !( $( ($traits) )&+ ));
    };
}

/// Asserts that the type does **not** implement _all_ of the given traits.
///
/// This macro has been deprecated in favor of
/// [`assert_impl_not_all!`](macro.assert_impl_not_all.html).
#[deprecated(
    since = "1.2.0",
    note = "Please use the 'assert_impl_not_all' macro instead"
)]
#[macro_export(local_inner_macros)]
macro_rules! assert_not_impl_all {
    ($($t:tt)*) => {
        assert_impl_not_all!($($t)*);
    };
}

/// Asserts that the type does **not** implement _any_ of the given traits.
///
/// This can be used to ensure types do not implement auto traits such as
/// [`Send`] and [`Sync`], as well as traits with [blanket `impl`s][blanket].
///
/// This macro causes a compilation failure if any of the provided individual
/// traits are implemented for the type. If you want to check that a combination
/// of traits is not implemented you should invoke [`assert_impl_not_all!`]
/// instead. For single traits both macros behave the same.
///
/// # Examples
///
/// If `u32` were to implement `Into` conversions for `usize` _and_ for `u8`,
/// the following would fail to compile:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_not_any!(u32: Into<usize>, Into<u8>);
/// ```
///
/// This is also good for simple one-off cases:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_not_any!(&'static mut u8: Copy);
/// ```
///
/// The following example fails to compile since `u32` can be converted into
/// `u64` even though it can not be converted into a `u16`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl_not_any!(u32: Into<u64>, Into<u16>);
/// ```
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [`assert_impl_not_all!`]: macro.assert_impl_not_all.html
/// [blanket]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
#[macro_export(local_inner_macros)]
macro_rules! assert_impl_not_any {
    ($ty:ty: $($traits:path),+ $(,)?) => {
        assert_impl!($ty: !( $( ($traits) )|+ ));
    };
}

/// Asserts that the type does **not** implement _any_ of the given traits.
///
/// This macro has been deprecated in favor of
/// [`assert_impl_not_any!`](macro.assert_impl_not_any.html).
#[deprecated(
    since = "1.2.0",
    note = "Please use the 'assert_impl_not_any' macro instead"
)]
#[macro_export(local_inner_macros)]
macro_rules! assert_not_impl_any {
    ($($t:tt)*) => {
        assert_impl_not_any!($($t)*);
    };
}

/// Asserts that the type implements a logical trait expression.
///
/// This macro causes a compilation failure if the expression is not satisfied.
///
/// See [`does_impl!`](macro.does_impl.html) for simply getting a [`bool`] from
/// this condition without asserting it.
///
/// # Syntax
///
/// ```skip
/// assert_impl!(<type>: <trait_expr>);
/// assert_impl!(for(<type>: <bounds>) <type>: <trait_expr>);
/// ```
///
/// where:
///
/// - `<type>` is a type (that must not depend on a generic parameter)
///
/// - `<trait_expr>` is an expression made out of trait names, combined with `!`
///   for negation, `&` for conjunction, `|` for disjunction and parentheses for
///   grouping.
///
/// - `<bounds>` is a trait bounds expression.
///
/// For technical reasons:
///
/// - Traits (like `Into<u8>`) that are not a single identifier must be
///   surrounded by parentheses.
///
/// - The usual operator priority is not respected: `x & y | z` is parsed as
///   `x & (y | z)`.
///
/// # Examples
///
/// If `u32` were to implement `Into` conversions for `usize` _and_ for `u8`,
/// the following would fail to compile:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(u32: !((Into<usize>) & (Into<u8>)));
/// ```
///
/// Check that a type is [`Send`] but not [`Sync`].
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// use std::cell::Cell;
///
/// assert_impl!(Cell<u32>: Send & !Sync);
/// ```
///
/// Check simple one-off cases:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(&'static mut u8: !Copy);
/// ```
///
/// Check that a type is _always_ [`Clone`] even when its parameter isn't:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// use std::rc::Rc;
///
/// assert_impl!(for(T) Rc<T>: Clone);
/// ```
///
/// The following example fails to compile since `u64` cannot be converted into
/// either `u32` or `u16`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(u64: (Into<u32>) | (Into<u16>));
/// ```
///
/// [`bool`]: https://doc.rust-lang.org/std/primitive.bool.html
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
#[macro_export(local_inner_macros)]
macro_rules! assert_impl {
    (for($($generic:tt)*) $ty:ty: $($rest:tt)*) => {
        const _: () = {
            fn assert_impl<$($generic)*>() {
                // Construct an expression using `True`/`False` and their
                // operators, that corresponds to the provided expression.
                let _: $crate::True = $crate::_does_impl!($ty: $($rest)*);
            }
        };
    };
    ($ty:ty: $($rest:tt)*) => {
        // Construct an expression using `True`/`False` and their operators,
        // that corresponds to the provided expression.
        const _: $crate::True = $crate::_does_impl!($ty: $($rest)*);
    };
}
