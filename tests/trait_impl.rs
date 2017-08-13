#![no_std]

#[macro_use]
extern crate static_assertions;

assert_impl!(byte; u8, Send, Sync);

#[test]
fn str_impl() {
    assert_impl!(str, Send, Sync, AsRef<[u8]>);
}
