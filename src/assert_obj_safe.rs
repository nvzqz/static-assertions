/// Asserts that the traits are [object-safe][object].
///
/// This is useful for when changes are made to a trait that accidentally
/// prevent it from being used as an [object]. Such a case would be adding a
/// generic method and forgetting to add `where Self: Sized` after it. If left
/// unnoticed, that mistake will affect crate users and break both forward and
/// backward compatibility.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
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
/// Generics without `where Self: Sized` are not allowed in object-safe traits:
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
#[macro_export]
macro_rules! assert_obj_safe {
    ($($xs:ty),+ $(,)*) => {
        $(let _: &$xs;)+
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_obj_safe!($($xs)+); }
    };
}
