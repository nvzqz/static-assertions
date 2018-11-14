#![no_std]
#![cfg_attr(feature = "nightly", feature(underscore_const_names))]

#[macro_use]
extern crate static_assertions;

mod m {
    pub struct _Struct<T: ?Sized> { pub nul: (), pub inner: T }
}

use m::_Struct as _Reused;

enum _Thing {
    A { x: u8 }
}

#[cfg(not(feature = "nightly"))]
mod stable {
    use super::*;
    assert_fields!(x; m::_Struct<str>, inner, nul);
    assert_fields!(y; _Reused<Send>, inner);
    assert_fields!(z; _Thing::A, x);
}

#[cfg(feature = "nightly")]
mod nightly {
    use super::*;
    assert_fields!(m::_Struct<str>, inner, nul);
    assert_fields!(_Reused<Send>, inner);
    assert_fields!(_Thing::A, x);
}
