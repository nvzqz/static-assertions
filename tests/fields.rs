#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

mod m {
    pub struct _Struct<T: ?Sized> { pub nul: (), pub inner: T }
}

use m::_Struct as _Reused;

enum _Thing {
    A { x: u8 }
}

assert_fields!(m::_Struct<str>, inner, nul);
assert_fields!(_Reused<dyn Send>, inner);
assert_fields!(_Thing::A, x);
