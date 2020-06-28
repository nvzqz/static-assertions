
/// Asserts that the type is [covariant] over the given lifetime.
///
/// For testing contravariance, also see [`assert_is_contravariant!`].
///
/// # Examples
///
/// This can ensure that any type has the expected variance for lifetime / type parameters.
/// This can be especially useful when making data structures:
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// struct Foo<T> {
///     x: *const T
/// }
///
/// assert_is_covariant!{
///     for[T] (Foo<&'a T>) over 'a
/// }
/// ```
/// Above, `Foo`'s variance over `T` is tested by using `&'a T` in its place, and testing variance
/// over `'a`.
///
///
/// You don't have to include the `for[...]` clause if it would be empty:
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_is_covariant!{
///     (fn() -> &'a i32) over 'a
/// }
/// ```
///
/// The following example fails to compile because `&mut T` is invariant over `T` (in this case,
/// `&'b i32`.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_is_covariant! {
///     for['a] (&'a mut &'b i32) over 'b
/// }
/// ```
///
/// [covariant]: https://doc.rust-lang.org/nomicon/subtyping.html#variance
/// [`assert_is_contravariant!`]: macro.assert_is_contravariant.html
#[macro_export]
macro_rules! assert_is_covariant {
    (for[$($gen_params:tt)*] ($type_name:ty) over $lf:lifetime) => {
        const _: () = {
            struct Cov<$lf, $($gen_params)*>($type_name);
            fn test_cov<'__a: '__b, '__b, $($gen_params)*>(
                subtype: *const Cov<'__a, $($gen_params)*>,
                mut _supertype: *const Cov<'__b, $($gen_params)*>,
            ) {
                _supertype = subtype;
            }
        };
    };

    (($type_name:ty) over $lf:lifetime) => {
        assert_is_covariant!(for[] ($type_name) over $lf);
    };
}

/// Asserts that the type is [contravariant] over the given lifetime.
///
/// For testing covariance, also see [`assert_is_covariant!`].
///
/// **Note:** contravariance is extremely rare, and only ever occurs with `fn` types taking
/// parameters with specific lifetimes.
///
/// # Examples
///
/// This can ensure that any type has the expected variance for lifetime / type parameters.
/// This can be especially useful when making data structures:
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// struct Foo<'a, T> {
///     x: fn(&'a T) -> bool,
/// }
///
/// assert_is_contravariant!{
///     for[T] (Foo<'a, T>) over 'a
/// }
/// ```
///
///
/// You don't have to include the `for[...]` clause if it would be empty:
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_is_contravariant!{
///     (fn(&'a i32)) over 'a
/// }
/// ```
///
/// The following example fails to compile because `&'a T` is covariant, not contravariant, over
/// `'a`.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_is_contravariant! {
///     (&'a mut f64) over 'a
/// }
/// ```
///
/// [contravariant]: https://doc.rust-lang.org/nomicon/subtyping.html#variance
/// [`assert_is_covariant!`]: macro.assert_is_covariant.html
#[macro_export]
macro_rules! assert_is_contravariant {
    (for[$($gen_params:tt)*] ($type_name:ty) over $lf:lifetime) => {
        const _: () = {
            struct Contra<$lf, $($gen_params)*>($type_name);
            fn test_contra<'__a: '__b, '__b, $($gen_params)*>(
                mut _subtype: *const Contra<'__a, $($gen_params)*>,
                supertype: *const Contra<'__b, $($gen_params)*>,
            ) {
                _subtype = supertype;
            }
        };
    };

    (($type_name:ty) over $lf:lifetime) => {
        assert_is_contravariant!(for[] ($type_name) over $lf);
    };
}