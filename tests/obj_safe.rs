#![no_std]
#![cfg_attr(feature = "nightly", feature(underscore_const_names))]

#[macro_use]
extern crate static_assertions;

use core::fmt::Debug;

#[cfg(not(feature = "nightly"))]
mod stable {
    use super::*;
    assert_obj_safe!(core_types; Debug, Send, Sync);
}

#[cfg(feature = "nightly")]
mod nightly {
    use super::*;
    assert_obj_safe!(Debug, Send, Sync);
}

#[test]
fn test_obj_safety() {
    trait ObjSafe {}

    assert_obj_safe!(ObjSafe);
}
