#![no_std]
#![cfg_attr(feature = "nightly", feature(underscore_const_names))]

#[macro_use]
extern crate static_assertions;

use core::ops::Range;

trait Tri<A: ?Sized, B: ?Sized, C: ?Sized> {}

impl<T, A: ?Sized, B: ?Sized, C: ?Sized> Tri<A, B, C> for T {}

#[cfg(not(feature = "nightly"))]
mod stable {
    use super::*;
    assert_impl!(tri; u64, Tri<[&'static u8], Tri<Send, Sync, str>, (u16, u16)>);
    assert_impl!(byte; u8, Send, Sync);
    assert_impl!(iter; &[u8], IntoIterator /* TODO: <Item=&u8> */);
    assert_impl!(range; Range<u8>, Iterator<Item=u8>);
    assert_impl!(slice; [u8], Send, Sync, AsRef<[u8]>);
}

#[cfg(feature = "nightly")]
mod nightly {
    use super::*;
    assert_impl!(u64, Tri<[&'static u8], Tri<Send, Sync, str>, (u16, u16)>);
    assert_impl!(u8, Send, Sync);
    assert_impl!(&[u8], IntoIterator /* TODO: <Item=&u8> */);
    assert_impl!(Range<u8>, Iterator<Item=u8>);
    assert_impl!([u8], Send, Sync, AsRef<[u8]>);
}

#[test]
fn str_impl() {
    assert_impl!(str, Send, Sync, AsRef<[u8]>);
}
