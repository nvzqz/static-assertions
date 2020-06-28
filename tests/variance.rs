#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

assert_is_covariant! {
    for['a, T] (&'a &'b T) over 'b
}

assert_is_covariant! {
    for['f, T] (core::ptr::NonNull<&'f &'a T>) over 'a
}

assert_is_covariant! {
    (&'a ()) over 'a
}

assert_is_contravariant! {
    (fn(&'a i32, &'a f64)) over 'a
}

assert_is_contravariant! {
    for[T] (fn(*const &'a ()) -> T) over 'a
}

assert_is_contravariant! {
    for[T] (*const fn(&'a T)) over 'a
}
