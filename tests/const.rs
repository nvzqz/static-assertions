#![no_std]
#![deny(dead_code)]

#[macro_use]
extern crate static_assertions;

assert_const!(less_than; 20 < 1000);
assert_const_eq!(twenty; 20, 30 - 10, 10 + 10, 10 * 2);

#[test]
fn assert_const() {
    const FIVE: usize = 5;

    assert_const!(FIVE * 2 == 10);
    assert_const!(FIVE > 2);
}
