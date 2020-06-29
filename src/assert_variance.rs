/// Asserts that the type is [covariant] over the given lifetime or type parameter.
///
/// For testing contravariance, see [`assert_is_contravariant!`].
///
/// # Examples
///
/// This can ensure that any type has the expected variance for lifetime or type parameters.
/// This can be especially useful when making data structures:
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// struct Foo<T> {
///     x: *const T
/// }
///
/// assert_is_covariant! {
///     (Foo<T>) over T
/// }
/// ```
///
/// Testing generics can be done with the `for[]` clause. Note that the type or lifetime parameter
/// being tested must not appear in the `for[]` clause.
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_is_covariant! {
///     for['a, T] (&'a &'b T) over 'b
/// }
/// ```
///
///
/// The following example fails to compile because `&mut T` is invariant over `T` (in this case,
/// `&'b i32`).
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// // WILL NOT COMPILE
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

    // This works because `&'a ()` is always covariant over `'a`. As a result, the subtyping
    // relation between any `&'x ()` and `&'y ()` always matches the relation between lifetimes `'x`
    // and `'y`. Therefore, testing variance over a type parameter `T` can be replaced by testing
    // variance over lifetime `'a` in `&'a ()`.
    // Even though this only checks cases where T is a reference, since a type constructor can be
    // ONLY covariant, contravariant, or invariant over a type parameter, if it is works in this case
    // it proves that the type is covariant in all cases.
    (for[$($($gen_params:tt)+)?] ($type_name:ty) over $type_param:ident) => {
        const _: () = {
            type Transform<$($($gen_params)+,)? $type_param> = $type_name;

            assert_is_covariant!{
                for[$($($gen_params)+)?] (Transform<$($($gen_params)+,)? &'__a ()>) over '__a
            }
        };
    };

    (($type_name:ty) over $($rest:tt)*) => {
        assert_is_covariant!(for[] ($type_name) over $($rest)*);
    };
}

/// Asserts that the type is [contravariant] over the given lifetime or type parameter.
///
/// For testing covariance, see [`assert_is_covariant!`].
///
/// **Note:** contravariance is extremely rare, and only ever occurs with `fn` types taking
/// parameters with specific lifetimes.
///
/// # Examples
///
/// This can ensure that any type has the expected variance for lifetime or type parameters.
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
/// The `for[...]` clause is unnecessary if it would be empty:
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_is_contravariant!{
///     (fn(T)) over T
/// }
/// ```
///
/// The following example fails to compile because `&'a mut T` is covariant, not contravariant,
/// over `'a`.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// // WILL NOT COMPILE
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

    // for info on why this works, see the implementation of assert_is_covariant
    (for[$($($gen_params:tt)+)?] ($type_name:ty) over $type_param:ident) => {
        const _: () = {
            type Transform<$($($gen_params)+,)? $type_param> = $type_name;

            assert_is_contravariant!{
                for[$($($gen_params)+)?] (Transform<$($($gen_params)+,)? &'__a ()>) over '__a
            }
        };
    };

    (($type_name:ty) over $($rest:tt)*) => {
        assert_is_contravariant!(for[] ($type_name) over $($rest)*);
    };
}
