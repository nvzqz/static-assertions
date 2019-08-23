#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

assert_ne_type!(u8, u16, u32);
