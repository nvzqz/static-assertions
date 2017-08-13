#![no_std]
#![deny(dead_code, non_camel_case_types)]
#![cfg_attr(feature = "const_fn", feature(const_fn))]

#[macro_use]
extern crate static_assertions;

const_assert!(less_than; 20 < 1000);
const_assert_eq!(twenty; 20, 30 - 10, 10 + 10, 10 * 2);

#[cfg(feature = "failure")]
const_assert!(less_than; true);

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
