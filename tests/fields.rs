#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

mod m {
    pub struct _Struct<T: ?Sized> { pub nul: (), pub inner: T }
}

use m::_Struct as _Reused;

#[allow(dead_code)]
enum _Thing {
    A { x: u8, y: u8 },
    B(u8),
}

assert_fields!(m::_Struct<str>, inner, nul);

assert_fields!(_Reused<dyn Send>, inner);

assert_fields!(_Thing::A, x);
assert_fields!(_Thing::A, x, x);
assert_fields!(_Thing::A, x, y, x);

// TODO: Make tuple field access possible
// assert_fields!(_Thing::B, 0);
