#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(underscore_const_names))]

#[macro_use]
extern crate static_assertions;

#[cfg(not(feature = "nightly"))]
mod stable {
    assert_ne_type!(int; u8, u16, u32);
}

#[cfg(feature = "nightly")]
mod nightly {
    assert_ne_type!(u8, u16, u32);
}
