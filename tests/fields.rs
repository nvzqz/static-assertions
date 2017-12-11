#![no_std]

#[macro_use]
extern crate static_assertions;

mod m {
    pub struct _Struct<T: ?Sized> { pub nul: (), pub inner: T }
}

use m::_Struct as _Reused;

enum _Thing {
    A { x: u8 }
}

assert_fields!(x; m::_Struct<str>, inner, nul);
assert_fields!(y; _Reused<Send>, inner);
assert_fields!(z; _Thing::A, x);
