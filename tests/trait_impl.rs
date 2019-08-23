#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

use core::ops::Range;

trait Tri<A: ?Sized, B: ?Sized, C: ?Sized> {}

impl<T, A: ?Sized, B: ?Sized, C: ?Sized> Tri<A, B, C> for T {}

assert_impl_all!(u64: Tri<[&'static u8], dyn Tri<dyn Send, dyn Sync, str>, (u16, u16)>);
assert_impl_all!(u8: Send, Sync);
assert_impl_all!(&'static [u8]: IntoIterator<Item=&'static u8>);
assert_impl_all!(Range<u8>: Iterator<Item=u8>);
assert_impl_all!([u8]: Send, Sync, AsRef<[u8]>);
assert_impl_all!(str: Send, Sync, AsRef<[u8]>,);
