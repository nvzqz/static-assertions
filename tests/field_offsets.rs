#![no_std]
#![allow(dead_code)]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

#[repr(C)]
struct Foo(u32, u64);

assert_field_offsets!(Foo {
    0: == 0,
    1: == 8,
});
