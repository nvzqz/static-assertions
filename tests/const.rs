#![no_std]
#![cfg_attr(feature = "nightly", feature(underscore_const_names))]

#[macro_use]
extern crate static_assertions;

#[cfg(not(feature = "nightly"))]
mod stable {
    const_assert!(less_than; 20 < 1000);
    const_assert_eq!(twenty; 20, 30 - 10, 10 + 10, 10 * 2);
}

#[cfg(feature = "nightly")]
mod nightly {
    const_assert!(true, true != false);
    const_assert_eq!(false, false);
}

#[test]
fn const_assert() {
    #[cfg_attr(feature = "nightly", allow(dead_code))]
    const FIVE: usize = 5;

    const_assert!(FIVE * 2 == 10);
    const_assert!(FIVE > 2);
}
