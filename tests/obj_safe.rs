#![no_std]

#[macro_use]
extern crate static_assertions;

use core::fmt::Debug;

assert_obj_safe!(core_types; Debug, Send, Sync);

#[test]
fn test_obj_safety() {
    trait ObjSafe {}

    assert_obj_safe!(ObjSafe);
}
