/// Asserts that the trait is a child of all of the other traits.
///
/// # Examples
///
/// All types that implement [`Copy`] must implement [`Clone`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_super_trait_all!(Copy: Clone);
/// ```
///
/// All types that implement [`Ord`] must implement [`PartialEq`], [`Eq`], and
/// [`PartialOrd`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_super_trait_all!(Ord: PartialEq, Eq, PartialOrd);
/// ```
///
/// The following example fails to compile because [`Eq`] is not required for
/// [`PartialOrd`]:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_super_trait_all!(PartialOrd: Eq);
/// ```
///
/// [`Copy`]:       https://doc.rust-lang.org/std/marker/trait.Copy.html
/// [`Clone`]:      https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [`Ord`]:        https://doc.rust-lang.org/std/cmp/trait.Ord.html
/// [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
/// [`Eq`]:         https://doc.rust-lang.org/std/cmp/trait.Eq.html
/// [`PartialEq`]:  https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
#[macro_export]
macro_rules! assert_super_trait_all {
    ($sub:path: $($super:path),+ $(,)?) => {
        const _: () = {
            $({
                #[allow(non_camel_case_types)]
                trait __Impl_Implication: $super {}

                impl<T: $sub> __Impl_Implication for T {}
            })+
        };
    };
}
