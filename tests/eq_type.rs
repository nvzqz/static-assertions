#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(underscore_const_names))]

#[macro_use]
extern crate static_assertions;

#[allow(dead_code)]
type X = u8;

#[cfg(not(feature = "nightly"))]
mod stable {
    assert_eq_type!(byte; super::X, u8, (super::X));
}

#[cfg(feature = "nightly")]
mod nightly {
    assert_eq_type!(super::X, u8, (super::X));
}

#[allow(dead_code)]
fn test() {
    assert_eq_type!([u8], [u8]);
}
