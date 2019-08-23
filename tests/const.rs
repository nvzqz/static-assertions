#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

const_assert!(true && (true != false));
const_assert!((true && true) != false);
const_assert_eq!(false, false);

#[allow(dead_code)]
const FIVE: usize = 5;

const_assert!(FIVE * 2 == 10);
const_assert!(FIVE > 2);
