/// Returns `true` if the type does implement a logical trait expression.
///
/// # Examples
///
/// One can mimic `assert_impl!` using this macro:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// const CONDITION: bool = does_impl!(u32: From<u8>);
///
/// const_assert!(CONDITION);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! does_impl {
    ($ty:ty: $($trait_expr:tt)+) => {
        _does_impl!($ty: $($trait_expr)+).value()
    };
}

/// Returns `True` or `False` depending on whether the given type implements the
/// given trait boolean expression. Can be used in const contexts if it doesn't
/// depend on outer generic parameters.
///
/// This is the core of `assert_impl`.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! _does_impl {
    ($ty:ty: $($rest:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::{
            _bool::{True, False},
            _core::{marker::PhantomData, ops::Deref},
        };

        // Fallback trait that returns false if the type does not implement a
        // given trait.
        trait DoesntImpl {
            const DOES_IMPL: False = False;
        }
        impl<T: ?Sized> DoesntImpl for T {}

        // Construct an expression using `True`/`False` and their operators,
        // that corresponds to the provided expression.
        *_does_impl!(@boolexpr($ty,) $($rest)*)
    }};

    (@boolexpr($($args:tt)*) ($($expr:tt)*)) => {
        _does_impl!(@boolexpr($($args)*) $($expr)*)
    };
    (@boolexpr($($args:tt)*) !($($expr:tt)*)) => {
        _does_impl!(@boolexpr($($args)*) $($expr)*).not()
    };
    (@boolexpr($($args:tt)*) ($($left:tt)*) | $($right:tt)*) => {{
        let left = _does_impl!(@boolexpr($($args)*) $($left)*);
        let right = _does_impl!(@boolexpr($($args)*) $($right)*);
        left.or(right)
    }};
    (@boolexpr($($args:tt)*) ($($left:tt)*) & $($right:tt)*) => {{
        let left = _does_impl!(@boolexpr($($args)*) $($left)*);
        let right = _does_impl!(@boolexpr($($args)*) $($right)*);
        left.and(right)
    }};
    (@boolexpr($($args:tt)*) !($($left:tt)*) | $($right:tt)*) => {{
        _does_impl!(@boolexpr($($args)*) (!($($left)*)) | $($right)*)
    }};
    (@boolexpr($($args:tt)*) !($($left:tt)*) & $($right:tt)*) => {{
        _does_impl!(@boolexpr($($args)*) (!($($left)*)) & $($right)*)
    }};
    (@boolexpr($($args:tt)*) !$left:ident | $($right:tt)*) => {{
        _does_impl!(@boolexpr($($args)*) !($left) | $($right)*)
    }};
    (@boolexpr($($args:tt)*) !$left:ident & $($right:tt)*) => {{
        _does_impl!(@boolexpr($($args)*) !($left) & $($right)*)
    }};
    (@boolexpr($($args:tt)*) $left:ident | $($right:tt)*) => {
        _does_impl!(@boolexpr($($args)*) ($left) | $($right)*)
    };
    (@boolexpr($($args:tt)*) $left:ident & $($right:tt)*) => {{
        _does_impl!(@boolexpr($($args)*) ($left) & $($right)*)
    }};
    (@boolexpr($($args:tt)*) !$expr:ident) => {
        _does_impl!(@boolexpr($($args)*) !($expr))
    };
    (@boolexpr($($args:tt)*) !$expr:path) => {
        _does_impl!(@boolexpr($($args)*) !($expr))
    };
    (@boolexpr($($args:tt)*) $expr:ident) => {
        _does_impl!(@base($($args)*) $expr)
    };
    (@boolexpr($($args:tt)*) $expr:path) => {
        _does_impl!(@base($($args)*) $expr)
    };

    (@base($ty:ty, $($args:tt)*) $($trait:tt)*) => {{
        // Base case: computes whether `ty` implements `trait`.
        struct Wrapper<T: ?Sized>(PhantomData<T>);

        #[allow(dead_code)]
        impl<T: ?Sized + $($trait)*> Wrapper<T> {
            const DOES_IMPL: True = True;
        }

        // If `$type: $trait`, the `_does_impl` inherent method on `Wrapper`
        // will be called, and return `True`. Otherwise, the trait method will
        // be called, which returns `False`.
        &<Wrapper<$ty>>::DOES_IMPL
    }};
}
