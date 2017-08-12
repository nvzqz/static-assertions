#![no_std]

#[macro_use]
extern crate static_assertions;

#[test]
fn const_assert() {
    const FIVE: usize = 5;

    const_assert!(FIVE * 2 == 10);
    const_assert!(FIVE > 2);
}
