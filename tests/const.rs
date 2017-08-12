#![no_std]

#![cfg_attr(feature = "const_fn", feature(const_fn))]

#[macro_use]
extern crate static_assertions;

#[test]
fn const_assert() {
    const FIVE: usize = 5;

    const_assert!(FIVE * 2 == 10);
    const_assert!(FIVE > 2);
}

#[cfg(feature = "const_fn")]
#[test]
fn const_fn() {
    const VALUE: usize = 4;

    const fn value() -> usize {
        VALUE
    }

    const_assert!(value() == VALUE);
}

#[cfg(feature = "nightly")]
#[test]
fn assoc_const() {
    const FOUR: usize = 4;
    trait Assoc { const VAL: usize; }
    struct Concrete;

    impl Assoc for Concrete {
        const VAL: usize = FOUR;
    }

    const_assert!(Concrete::VAL == FOUR);
}

// This test is expected to fail at compile-time
#[cfg(feature = "failure")]
#[test]
fn test_fail() {
    const_assert!(1 + 1 != 2);
    const_assert!(1 < 1);
}
