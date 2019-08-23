#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

assert_obj_safe!(core::fmt::Debug, Send, Sync);

trait ObjSafe {}
assert_obj_safe!(ObjSafe);
