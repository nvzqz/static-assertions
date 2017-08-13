#![no_std]

#[macro_use]
extern crate static_assertions;

assert_impl!(byte; u8, Send, Sync);
assert_impl!(iter; &[u8], IntoIterator /* TODO: <Item=&u8> */);
assert_impl!(slice; [u8], Send, Sync, AsRef<[u8]>);

#[cfg(feature = "failure")]
assert_impl!(ptr; *const u8, Send, Sync);

#[test]
fn str_impl() {
    assert_impl!(str, Send, Sync, AsRef<[u8]>);
}
