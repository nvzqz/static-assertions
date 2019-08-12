// FIXME: Link below is required to render in index
/// Asserts that the traits are [object-safe](https://doc.rust-lang.org/book/2018-edition/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects).
///
/// This is useful for when changes are made to a trait that accidentally
/// prevent it from being used as an [object]. Such a case would be adding a
/// generic method and forgetting to add `where Self: Sized` after it. If left
/// unnoticed, that mistake will affect crate users and break both forward and
/// backward compatibility.
///
/// # Examples
///
/// When exposing a public API, it's important that traits that could previously
/// use dynamic dispatch can still do so in future compatible crate versions.
///
#[cfg_attr(feature = "nightly", doc = "```ignore")]
#[cfg_attr(not(feature = "nightly"), doc = "```")]
/// # #[macro_use] extern crate static_assertions;
/// // Requires a unique label in module scope
/// assert_obj_safe!(basic; Send, Sync, AsRef<str>);
///
/// mod inner {
///     // Works with traits that are not in the calling module
///     pub trait BasicTrait {
///         fn foo(&self);
///     }
/// }
///
/// trait MySafeTrait {
///     fn bar(&self) -> u32;
/// }
///
/// fn main() {
///     assert_obj_safe!(MySafeTrait);
///     assert_obj_safe!(inner::BasicTrait);
/// }
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
/// use std::fmt;
///
/// assert_obj_safe!(fmt::Write);
///
/// fn main() {
///     assert_obj_safe!(fmt::Debug);
/// }
/// ```
///
/// Raw pointers cannot be sent between threads safely:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_impl!(*const u8, Send);
/// # }
/// ```
///
/// Generics without `where Self: Sized` are not allowed in
/// [object-safe][object] trait methods:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// trait MyUnsafeTrait {
///     fn baz<T>(&self) -> T;
/// }
///
/// # fn main() {
/// assert_obj_safe!(MyUnsafeTrait);
/// # }
/// ```
///
/// [object]: https://doc.rust-lang.org/book/2018-edition/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects
#[macro_export(local_inner_macros)]
macro_rules! assert_obj_safe {
    ($($xs:tt)+) => { _assert_obj_safe!($($xs)+); };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export(local_inner_macros)]
macro_rules! _assert_obj_safe {
    ($($xs:ty),+ $(,)*) => {
        $(const _: Option<&$xs> = None;)+
    };
}

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export(local_inner_macros)]
macro_rules! _assert_obj_safe {
    ($($xs:ty),+ $(,)*) => {
        $(let _: &$xs;)+
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_obj_safe!($($xs)+); }
    };
}
