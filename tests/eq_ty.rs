#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(underscore_const_names))]

#[macro_use]
extern crate static_assertions;

type X = u8;

#[cfg(not(feature = "nightly"))]
mod stable {
    assert_eq_type!(byte; super::X, u8, (super::X));
}

#[cfg(feature = "nightly")]
mod nightly {
    assert_eq_type!(super::X, u8, (super::X));
}
