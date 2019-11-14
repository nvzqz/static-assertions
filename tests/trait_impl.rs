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

assert_impl_any!((): Send, Sync);
assert_impl_any!((): Send, From<u8>);
assert_impl_any!((): From<u8>, From<u16>, Send);

#[allow(dead_code)]
struct Foo;

trait A {}
trait B {}
trait C {}

impl B for Foo {}

assert_impl_one!(Foo: A, B);
assert_impl_one!(Foo: B, A);
assert_impl_one!(Foo: B, C);
assert_impl_one!(Foo: C, B);
assert_impl_one!(Foo: A, B, C);
assert_impl_one!(Foo: B, C, A);
assert_impl_one!(Foo: C, A, B);

#[derive(Clone)]
struct Test;

assert_impl!(u8: (From<u16>) || (Into<u16>));
assert_impl!((): (From<u8>) || (From<u16>) || Send);
assert_impl!((): (!From<u8>) && !(From<u16>) && Send);
assert_impl!((): Copy || Clone);
assert_impl!((): Copy && Clone);
assert_impl!(Test: Copy || Clone);
assert_impl!(Test: !Copy || Clone);
assert_impl!(Test: !Copy && Clone);
assert_impl!(Test: !Copy && (Clone));
assert_impl!(Test: !(Copy) && Clone);
assert_impl!(Test: !(!Clone));
assert_impl!(Test: !(Copy) && !(!Clone));
assert_impl!(Test: !(Copy && Clone));
assert_impl!(str: !Copy && !Clone);

#[derive(Clone)]
struct Box<T>(T);

assert_impl!(for(T: Clone) Box<T>: Clone);
assert_impl!(for(T: Clone + Send) Box<T>: Clone && Send);
assert_impl!(for(T) PhantomData<T>: Clone);
assert_impl!(for(T: Copy) T: Clone);
assert_impl!(for(T: ?Sized) T: Clone || !Clone);
