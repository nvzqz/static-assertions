#![no_std]

#[macro_use]
extern crate static_assertions;

/// A type that panics on drop.
struct PanicDrop<T>(T);

impl<T> Drop for PanicDrop<T> {
    fn drop(&mut self) {
        panic!("Dropped!");
    }
}

#[test]
fn test_eq_size() {
    assert_eq_size!([u8; 2], u16);
    assert_eq_size!([u8; 2], u16, (u8, u8));
    assert_eq_size!([u8; 4], u32, (u16, u8, u8), (u16, u16));
    // assert_eq_size!(u8, u16); // Fails to compile

    #[deny(unused_unsafe)]
    unsafe { assert_eq_size!(u8, u8); }
}

#[test]
fn test_eq_size_no_drop() {
    assert_eq_size!(u32, PanicDrop<u32>);
    assert_eq_size!(PanicDrop<u32>, u32);
    assert_eq_size!(PanicDrop<u32>, PanicDrop<u32>);
}
