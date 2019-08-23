#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

assert_eq_type!([u8], [u8]);

#[allow(dead_code)]
type X = u8;

mod m {
    assert_eq_type!(super::X, u8, (super::X));
}
