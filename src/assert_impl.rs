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
/// If `u32` were to implement `Into` conversions for `usize` and `u8`, the
/// following would fail to compile:
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
/// assert_impl_not_any!(&mut u8: Copy);
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

/// Asserts that the type implements a [logical trait expression].
///
/// See [`impls`] for simply getting a [`bool`] from this condition without
/// asserting it.
///
/// # Index
///
/// - [Syntax](#syntax)
/// - [Logical Trait Expression](#logical-trait-expression)
/// - [Examples](#examples)
///   - [Precedence and Nesting](#precedence-and-nesting)
///   - [Mutual Exclusion](#mutual-exclusion)
///   - [Generic Types](#generic-types)
///   - [Reference Types](#reference-types)
/// - [False Positives](#false-positives)
/// - [Limitations](#limitations)
///
/// # Syntax
///
/// ```skip
/// assert_impl!($type: $trait_expr);
/// assert_impl!(for($generic) $type: $trait_expr);
/// ```
///
/// where:
///
/// - `$type` is any Rust type in scope.
///   - Limitation: cannot depend on an external generic parameter.
///
/// - `$trait_expr` is [logical trait expression].
///   - Limitation: cannot depend on _any_ generic parameter.
///
/// - `$generic` is a set of [generic parameters][generics] usable by `$type`.
///
/// # Logical Trait Expression
///
/// In this macro, a trait should be thought of as a [`bool`] indicating whether
/// the given type implements it.
///
/// An expression can be formed from these trait operations:
///
/// - And (`&`): also known as [logical conjunction], this returns `true` if
///   **both** operands are `true`. This is usually defined in Rust via the
///   [`BitAnd`] trait.
///
/// - Or (`|`): also known as [logical disjunction], this returns `true` if
///   **either** of two operands is `true`. This is usually defined in Rust via
///   the [`BitOr`] trait.
///
/// - Exclusive-or (`^`): also known as [exclusive disjunction], this returns
///   `true` if **only one** of two operands is `true`. This is usually defined
///   in Rust via the [`BitXor`] trait.
///
/// - Not (`!`): a negation that returns `false` if the operand is `true`, or
///   `true` if the operand is `false`. This is usually defined in Rust via the
///   [`Not`] trait.
///
/// # Examples
///
/// This macro is very flexible and works in a variety of ways as described
/// below.
///
/// ## Precedence and Nesting
///
/// Trait operations abide by [Rust's expression precedence][precedence].
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(u8: Copy | Copy ^ Copy & Copy);
/// assert_impl!(u8: Copy & Copy ^ Copy | Copy);
/// ```
///
/// The first expression evaluated left-to-right fails:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(u8: ((Copy | Copy) ^ Copy) & Copy);
/// ```
///
/// The second expression evaluated right-to-left fails:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(u8: Copy & (Copy ^ (Copy | Copy)));
/// ```
///
/// ## Mutual Exclusion
///
/// Because exclusive-or (`^`) is a trait operation, we can check that a type
/// implements one of two traits, but not both:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// struct T;
///
/// trait Foo {}
/// trait Bar {}
///
/// impl Foo for T {}
///
/// assert_impl!(T: Foo ^ Bar);
/// ```
///
/// ## Generic Types
///
/// Check that a type is _always_ [`Clone`], even when its parameter isn't:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// use std::rc::Rc;
///
/// assert_impl!(for(T) Rc<T>: Clone);
/// ```
///
/// Check that a type is [`Send`] but not [`Sync`], even when its parameter is:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// use std::cell::Cell;
///
/// assert_impl!(for(T: Send + Sync) Cell<T>: Send & !Sync);
/// ```
///
/// ## Reference Types
///
/// If a type implements [`Sync`], then a reference to it implements [`Send`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(for(T: Sync) &T: Send);
/// ```
///
/// Something surprising to many Rust users is that [`&mut T`] _does not_
/// implement [`Copy`] _nor_ [`Clone`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(for(T) &mut T: !Copy & !Clone);
/// ```
///
/// Surely you're thinking now that this macro must be broken, because you've
/// been able to reuse [`&mut T`] throughout your lifetime with Rust. This works
/// because, in certain contexts, the compiler silently adds "re-borrows"
/// (`&mut *ref`) with a shorter lifetime and shadows the original. In reality,
/// [`&mut T`] is a move-only type.
///
/// # False Positives
///
/// There is a bug where asserting a generic type `A<T>` implements [`From<T>`]
/// or [`Into<T>`] will always pass.
///
/// For example, asserting that [`Rc<T>`] implements [`Into<T>`] passes when it
/// shouldn't:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// # use std::rc::Rc;
/// assert_impl!(for(T) Rc<T>: Into<T>);
/// ```
///
/// If we try to use [`Into<T>`], we can see that [`Rc<T>`] does not actually
/// implement it:
///
/// ```compile_fail
/// # use std::rc::Rc;
/// fn convert<T>(rc: Rc<T>) -> T {
///     Into::<T>::into(rc)
/// }
/// ```
///
/// # Limitations
///
/// The type cannot refer to an external generic parameter:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// # use std::rc::Rc;
/// fn test<T>() {
///     assert_impl!(Rc<T>: Clone);
/// }
/// ```
///
/// The trait cannot refer to any generic parameter:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_impl!(for(T, U: From<T>) T: Into<U>);
/// ```
///
/// [`impls`]: https://github.com/nvzqz/impls
/// [exclusive disjunction]: https://en.wikipedia.org/wiki/Exclusive_disjunction
/// [logical conjunction]: https://en.wikipedia.org/wiki/Logical_conjunction
/// [logical disjunction]: https://en.wikipedia.org/wiki/Logical_disjunction
/// [logical trait expression]: #logical-trait-expression
///
/// [generics]: https://doc.rust-lang.org/book/ch10-00-generics.html
/// [precedence]: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
///
/// [`&mut T`]:  https://doc.rust-lang.org/std/primitive.reference.html
/// [`BitAnd`]:  https://doc.rust-lang.org/std/ops/trait.BitAnd.html
/// [`BitOr`]:   https://doc.rust-lang.org/std/ops/trait.BitOr.html
/// [`BitXor`]:  https://doc.rust-lang.org/std/ops/trait.BitXor.html
/// [`bool`]:    https://doc.rust-lang.org/std/primitive.bool.html
/// [`Clone`]:   https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [`Copy`]:    https://doc.rust-lang.org/std/marker/trait.Copy.html
/// [`From`]:    https://doc.rust-lang.org/std/convert/trait.From.html
/// [`From<T>`]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [`Into`]:    https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`Into<T>`]: https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`Not`]:     https://doc.rust-lang.org/std/ops/trait.Not.html
/// [`Rc<T>`]:   https://doc.rust-lang.org/std/rc/struct.Rc.html
/// [`Send`]:    https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]:    https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [`u16`]:     https://doc.rust-lang.org/std/primitive.u16.html
/// [`u32`]:     https://doc.rust-lang.org/std/primitive.u32.html
/// [`u64`]:     https://doc.rust-lang.org/std/primitive.u64.html
/// [`u8`]:      https://doc.rust-lang.org/std/primitive.u8.html
/// [`usize`]:   https://doc.rust-lang.org/std/primitive.usize.html
#[macro_export(local_inner_macros)]
macro_rules! assert_impl {
    (for($($generic:tt)+) $ty:ty: $($trait_expr:tt)+) => {
        const _: () = {
            fn assert_impl<$($generic)+>() {
                // Construct an expression using `True`/`False` and their
                // operators, that corresponds to the provided expression.
                let _: $crate::True = $crate::_impls!($ty: $($trait_expr)+);
            }
        };
    };
    ($ty:ty: $($trait_expr:tt)+) => {
        // Construct an expression using `True`/`False` and their operators,
        // that corresponds to the provided expression.
        const _: fn() = || {
            let _: $crate::True = $crate::_impls!($ty: $($trait_expr)+);
        };
    };
}
