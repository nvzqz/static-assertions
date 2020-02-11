/// Returns `True` or `False` depending on whether the given type implements the
/// given trait boolean expression.
///
/// This is an adaptation of `impls` (https://github.com/nvzqz/impls), but this
/// results in a type mismatch on false instead of just returning a `const`
/// false. We can't use `impls` directly because it can't reference outer
/// generic types. However, this can because it doesn't try to evaluate the
/// expression in a `const _`. Instead, it just performs the operations as
/// non-const using the outputs of the `core::ops` traits.
///
/// Note: This cannot be used in a `const _` directly and instead must be used
/// indirectly inside of a function.
///
/// This is the core of `assert_impl`.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! _impls {
    // ONE: Turn `$trait` into `True` or `False` based on whether `$type`
    // implements it.
    ($type:ty: $(! !)* $trait:path) => {{
        // Do not import types in order to prevent trait name collisions.

        /// Fallback trait with `False` for `impls` if the type does not
        /// implement the given trait.
        trait __SA_DoesNotImpl {
            fn impls(&self) -> $crate::False {
                $crate::False
            }
        }
        impl<T: ?Sized> __SA_DoesNotImpl for T {}

        /// Concrete type with `True` for `impls` if the type implements the
        /// given trait. Otherwise, it falls back to `DoesNotImpl`.
        struct __SA_Wrapper<T: ?Sized>($crate::_core::marker::PhantomData<T>);

        #[allow(dead_code)]
        impl<T: ?Sized + $trait> __SA_Wrapper<T> {
            fn impls(&self) -> $crate::True {
                $crate::True
            }
        }

        let impls = __SA_Wrapper($crate::_core::marker::PhantomData::<$type>).impls();
        impls
    }};

    // HACK: The use of `({ let impls = ...; impls })` manages to make the
    // parser satisfied about the expression. Otherwise, weird errors happen
    // such as expecting `True` and getting `False` regardless of what the
    // expression evaluates to.

    // NOT
    ($type:ty: $(! !)* !$trait:path) => {
        ({
            let impls = _impls!($type: $trait);
            !impls
        })
    };

    // PAREN
    ($type:ty: $(! !)* ($($trait_expr:tt)+)) => {
        _impls!($type: $($trait_expr)+)
    };
    // PAREN+NOT
    ($type:ty: $(! !)* !($($trait_expr:tt)+)) => {
        ({
            let impls = _impls!($type: $($trait_expr)+);
            !impls
        })
    };
    // PAREN+OR
    ($type:ty: $(! !)* ($($t1:tt)+) | $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $($t1)+);
            impls
        })
        |
        _impls!($type: $($t2)+)
    };
    // PAREN+OR+NOT
    ($type:ty: $(! !)* !($($t1:tt)+) | $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $($t1)+);
            !impls
        })
        |
        _impls!($type: $($t2)+)
    };
    // PAREN+AND
    ($type:ty: $(! !)* ($($t1:tt)+) & $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $($t1)+);
            impls
        })
        &
        _impls!($type: $($t2)+)
    };
    // PAREN+AND+NOT
    ($type:ty: $(! !)* !($($t1:tt)+) & $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $($t1)+);
            !impls
        })
        &
        _impls!($type: $($t2)+)
    };
    // PAREN+XOR
    ($type:ty: $(! !)* ($($t1:tt)+) ^ $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $($t1)+);
            impls
        })
        ^
        _impls!($type: $($t2)+)
    };
    // PAREN+XOR+NOT
    ($type:ty: $(! !)* !($($t1:tt)+) ^ $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $($t1)+);
            !impls
        })
        ^
        _impls!($type: $($t2)+)
    };

    // OR: Any.
    ($type:ty: $(! !)* $t1:path | $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $t1);
            impls
        })
        |
        _impls! { $type: $($t2)+ }
    };
    // // OR+: Any.
    // ($type:ty: $($(!!)* $ts:path)|+) => {
    //     $(_impls!($type: $ts))|+
    // };
    // OR+NOT: Any.
    ($type:ty: $(! !)* !$t1:path | $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $t1);
            !impls
        })
        |
        _impls!($type: $($t2)+)
    };

    // AND: 0 lifetimes, 0 generics.
    ($type:ty: $(! !)* $t1:ident & $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $t1);
            impls
        })
        &
        _impls! { $type: $($t2)+ }
    };
    // AND+NOT: 0 lifetimes, 0 generics.
    ($type:ty: $(! !)* !$t1:ident & $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $t1);
            !impls
        })
        &
        _impls!($type: $($t2)+)
    };

    // AND: 1+ lifetimes, 0+ generics.
    (
        $type:ty: $(! !)*
        $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        &
        $($t2:tt)+
    ) => {
        ({
            let impls _impls!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >);
            impls
        })
        &
        _impls!($type: $($t2)+)
    };
    // AND+NOT: 1+ lifetimes, 0+ generics.
    (
        $type:ty: $(! !)*
        !$t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        &
        $($t2:tt)+
    ) => {
        ({
            let impls = _impls!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >);
            !impls
        })
        &
        _impls!($type: $($t2)+)
    };

    // AND: 0 lifetimes, 1+ generics.
    (
        $type:ty: $(! !)*
        $t1:ident < $($t1_generic:ty),+ $(,)? >
        &
        $($t2:tt)+
    ) => {
        ({
            let impls = _impls!($type: $t1 < $($t1_generic),+ >);
            impls
        })
        &
        _impls!($type: $($t2)+)
    };
    // AND+NOT: 0 lifetimes, 1+ generics.
    (
        $type:ty: $(! !)*
        !$t1:ident < $($t1_generic:ty),+ $(,)? >
        &
        $($t2:tt)+
    ) => {
        ({
            let impls = _impls!($type: $t1 < $($t1_generic),+ >);
            !impls
        })
        &
        _impls!($type: $($t2)+)
    };

    // XOR: 0 lifetimes, 0 generics.
    ($type:ty: $(! !)* $t1:ident ^ $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $t1);
            impls
        })
        ^
        _impls! { $type: $($t2)+ }
    };
    // XOR+NOT: 0 lifetimes, 0 generics.
    ($type:ty: $(! !)* !$t1:ident ^ $($t2:tt)+) => {
        ({
            let impls = _impls!($type: $t1);
            !impls
        })
        ^
        _impls!($type: $($t2)+)
    };

    // XOR: 1+ lifetimes, 0+ generics.
    (
        $type:ty: $(! !)*
        $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        ^
        $($t2:tt)+
    ) => {
        ({
            let impls = _impls!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >);
            impls
        })
        ^
        _impls!($type: $($t2)+)
    };
    // XOR+NOT: 1+ lifetimes, 0+ generics.
    (
        $type:ty: $(! !)*
        ! $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        ^
        $($t2:tt)+
    ) => {
        ({
            let impls = _impls!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >);
            !impls
        })
        ^
        _impls!($type: $($t2)+)
    };

    // XOR: 0 lifetimes, 1+ generics.
    (
        $type:ty: $(! !)*
        $t1:ident < $($t1_generic:ty),+ $(,)? >
        ^
        $($t2:tt)+
    ) => {
        ({
            let impls = _impls!($type: $t1 < $($t1_generic),+ >);
        })
        ^
        _impls!($type: $($t2)+)
    };
    // XOR+NOT: 0 lifetimes, 1+ generics.
    (
        $type:ty: $(! !)*
        ! $t1:ident < $($t1_generic:ty),+ $(,)? >
        ^
        $($t2:tt)+
    ) => {
        ({
            let impls = _impls!($type: $t1 < $($t1_generic),+ >);
            !impls
        })
        ^
        _impls!($type: $($t2)+)
    };
}
