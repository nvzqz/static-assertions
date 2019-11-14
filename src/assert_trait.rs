/// Asserts that the trait is a child of all of the other traits.
///
/// Related:
/// - [`assert_trait_super_all!`]
///
/// # Examples
///
/// All types that implement [`Copy`] must implement [`Clone`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_trait_sub_all!(Copy: Clone);
/// ```
///
/// All types that implement [`Ord`] must implement [`PartialEq`], [`Eq`], and
/// [`PartialOrd`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_trait_sub_all!(Ord: PartialEq, Eq, PartialOrd);
/// ```
///
/// The following example fails to compile because [`Eq`] is not required for
/// [`PartialOrd`]:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_trait_sub_all!(PartialOrd: Eq);
/// ```
///
/// [`assert_trait_super_all!`]: macro.assert_trait_super_all.html
///
/// [`Copy`]:       https://doc.rust-lang.org/std/marker/trait.Copy.html
/// [`Clone`]:      https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [`Ord`]:        https://doc.rust-lang.org/std/cmp/trait.Ord.html
/// [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
/// [`Eq`]:         https://doc.rust-lang.org/std/cmp/trait.Eq.html
/// [`PartialEq`]:  https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
#[macro_export]
macro_rules! assert_trait_sub_all {
    ($sub:path: $($super:path),+ $(,)?) => {
        const _: fn() = || {
            use $crate::_core::marker::PhantomData;
            use $crate::_core::ops::Deref;

            #[derive(Copy, Clone)]
            struct True;
            #[derive(Copy, Clone)]
            struct False;

            fn assert_true(_: True){}
            fn assert_false(_: False){}

            trait Not {
                type Result;
                fn not(self) -> Self::Result;
            }
            trait And<Other> {
                type Result;
                fn and(self, _: Other) -> Self::Result;
            }
            trait Or<Other> {
                type Result;
                fn or(self, _: Other) -> Self::Result;
            }

            impl Not for True {
                type Result = False;
                fn not(self) -> False { False }
            }
            impl Not for False {
                type Result = True;
                fn not(self) -> True { True }
            }

            impl<T> And<T> for True {
                type Result = T;
                fn and(self, other: T) -> Self::Result { other }
            }
            impl<T> And<T> for False {
                type Result = False;
                fn and(self, _: T) -> Self::Result { False }
            }
            impl<T> Or<T> for True {
                type Result = True;
                fn or(self, _: T) -> Self::Result { True }
            }
            impl<T> Or<T> for False {
                type Result = T;
                fn or(self, other: T) -> Self::Result { other }
            }

            trait DoesImpl {
                type Result;
                fn does_impl(&self) -> Self::Result;
            }

            struct Base;
            impl DoesImpl for Base {
                type Result = False;
                fn does_impl(&self) -> False {
                    False
                }
            }
            static BASE: Base = Base;


            fn forall<T: $sub>() {
                let result = True;

                $(let result = result.and({
                    struct Wrapper<T>(PhantomData<T>);

                    impl<T> Deref for Wrapper<T> {
                        type Target = Base;
                        fn deref(&self) -> &Self::Target {
                            &BASE
                        }
                    }

                    impl<T: $super> DoesImpl for Wrapper<T> {
                        type Result = True;
                        fn does_impl(&self) -> True {
                            True
                        }
                    }

                    // If `$type: $trait`, the `does_impl` method on `Wrapper` will be called, and return
                    // `True`. Otherwise, the compiler will try to deref and call the method on `Base`,
                    // which returns `False`.
                    Wrapper::<T>(PhantomData).does_impl()
                });)+

                assert_true(result)
            }
        };
    };
}

/// Asserts that the trait is a parent of all of the other traits.
///
/// Related:
/// - [`assert_trait_sub_all!`]
///
/// # Examples
///
/// With this, traits `A` and `B` can both be tested to require [`Copy`] on a
/// single line:
///
/// ```
/// # use static_assertions::assert_trait_super_all;
/// trait A: Copy {}
/// trait B: Copy {}
///
/// assert_trait_super_all!(Copy: A, B);
/// ```
///
/// Otherwise, each sub-trait would require its own call to
/// [`assert_trait_sub_all!`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// # trait A: Copy {}
/// # trait B: Copy {}
/// assert_trait_sub_all!(A: Copy);
/// assert_trait_sub_all!(B: Copy);
/// ```
///
/// The following example fails to compile because trait `C` does not require
/// [`Copy`]:
///
/// ```compile_fail
/// # use static_assertions::assert_trait_super_all;
/// # trait A: Copy {}
/// # trait B: Copy {}
/// trait C {}
///
/// assert_trait_super_all!(Copy: A, B, C);
/// ```
///
/// [`assert_trait_sub_all!`]: macro.assert_trait_sub_all.html
///
/// [`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
#[macro_export(local_inner_macros)]
macro_rules! assert_trait_super_all {
    ($super:path: $($sub:path),+ $(,)?) => {
        $(assert_trait_sub_all!($sub: $super);)+
    };
}

/// Asserts that the trait is a child of one or more of the other traits.
///
/// Related:
/// - [`assert_impl_any!`]
///
/// # Examples
///
/// All types that implement [`Copy`] must implement [`Clone`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_trait_sub_any!(Copy: Clone);
/// ```
///
/// All types that implement [`Ord`] must implement [`Eq`], but don't have to implement [`Clone`]:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_trait_sub_any!(Ord: Eq, Clone);
/// ```
///
/// The following example fails to compile because neither [`Eq`] nor [`Clone`] are required for
/// [`PartialOrd`]:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_trait_sub_any!(PartialOrd: Eq, Clone);
/// ```
///
/// [`assert_impl_any!`]: macro.assert_impl_any.html
///
/// [`Copy`]:       https://doc.rust-lang.org/std/marker/trait.Copy.html
/// [`Clone`]:      https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [`Ord`]:        https://doc.rust-lang.org/std/cmp/trait.Ord.html
/// [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
/// [`Eq`]:         https://doc.rust-lang.org/std/cmp/trait.Eq.html
/// [`PartialEq`]:  https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
#[macro_export]
macro_rules! assert_trait_sub_any {
    ($sub:path: $($super:path),+ $(,)?) => {
        const _: fn() = || {
            use $crate::_core::marker::PhantomData;
            use $crate::_core::ops::Deref;

            #[derive(Copy, Clone)]
            struct True;
            #[derive(Copy, Clone)]
            struct False;

            fn assert_true(_: True){}
            fn assert_false(_: False){}

            trait Not {
                type Result;
                fn not(self) -> Self::Result;
            }
            trait And<Other> {
                type Result;
                fn and(self, _: Other) -> Self::Result;
            }
            trait Or<Other> {
                type Result;
                fn or(self, _: Other) -> Self::Result;
            }

            impl Not for True {
                type Result = False;
                fn not(self) -> False { False }
            }
            impl Not for False {
                type Result = True;
                fn not(self) -> True { True }
            }

            impl<T> And<T> for True {
                type Result = T;
                fn and(self, other: T) -> Self::Result { other }
            }
            impl<T> And<T> for False {
                type Result = False;
                fn and(self, _: T) -> Self::Result { False }
            }
            impl<T> Or<T> for True {
                type Result = True;
                fn or(self, _: T) -> Self::Result { True }
            }
            impl<T> Or<T> for False {
                type Result = T;
                fn or(self, other: T) -> Self::Result { other }
            }

            trait DoesImpl {
                type Result;
                fn does_impl(&self) -> Self::Result;
            }

            struct Base;
            impl DoesImpl for Base {
                type Result = False;
                fn does_impl(&self) -> False {
                    False
                }
            }
            static BASE: Base = Base;


            fn forall<T: $sub>() {
                let result = False;

                $(let result = result.or({
                    struct Wrapper<T>(PhantomData<T>);

                    impl<T> Deref for Wrapper<T> {
                        type Target = Base;
                        fn deref(&self) -> &Self::Target {
                            &BASE
                        }
                    }

                    impl<T: $super> DoesImpl for Wrapper<T> {
                        type Result = True;
                        fn does_impl(&self) -> True {
                            True
                        }
                    }

                    // If `$type: $trait`, the `does_impl` method on `Wrapper` will be called, and return
                    // `True`. Otherwise, the compiler will try to deref and call the method on `Base`,
                    // which returns `False`.
                    Wrapper::<T>(PhantomData).does_impl()
                });)+

                assert_true(result)
            }
        };
    };
}
