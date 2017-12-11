#![no_std]
#![deny(dead_code)]

#[macro_use]
extern crate static_assertions;

const_assert!(less_than; 20 < 1000);
const_assert_eq!(twenty; 20, 30 - 10, 10 + 10, 10 * 2);

#[test]
fn const_assert() {
    const FIVE: usize = 5;

    const_assert!(FIVE * 2 == 10);
    const_assert!(FIVE > 2);
}
