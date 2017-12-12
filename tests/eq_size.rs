#![no_std]

#[macro_use]
extern crate static_assertions;

assert_eq_size!(byte; u8, u8, (u8,), [u8; 1]);

mod assoc_type {
    trait Trait {
        type AssocItem;
    }

    impl<T> Trait for T {
        type AssocItem = Self;
    }

    struct Value;

    impl Value {
        assert_eq_size!(test; <Self as Trait>::AssocItem, Self);
    }
}

// Placed in separate module so that DropCounter's fields are private
mod dc {
    /// A type that acts somewhat of a reference counter.
    pub struct DropCounter<'a> {
        count: &'a mut i32
    }

    impl<'a> DropCounter<'a> {
        pub fn new(count: &'a mut i32) -> DropCounter<'a> {
            *count += 1;
            DropCounter { count: count }
        }

        pub fn count(&self) -> i32 {
            *self.count
        }
    }

    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            *self.count -= 1
        }
    }
}
use dc::*;

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

    assert_eq_size_val!([0u8; 2], 0u16);
    assert_eq_size_val!([0u8; 2], 0u16, (0u8, 0u8));
    assert_eq_size_val!([0u8; 4], 0u32, (0u16, 0u8, 0u8), (0u16, 0u16));

    #[deny(unused_unsafe)]
    unsafe {
        assert_eq_size!(u8, u8);
        assert_eq_size_val!(0u8, 0u8);
    }

    let x = &mut 0;
    assert_eq_size_ptr!(x, &0);
    *x = 20;
    assert_eq_size_ptr!(x, &0);

    // Should fail to compile (un-comment to test manually):
    // assert_eq_size!(u8, u16);
    // assert_eq_size_val!(0u8, 0u16);
}

#[test]
fn test_eq_size_no_drop() {
    assert_eq_size!(u32, PanicDrop<u32>);
    assert_eq_size!(PanicDrop<u32>, u32);
    assert_eq_size!(PanicDrop<u32>, PanicDrop<u32>);
}

#[test]
fn test_eq_size_drop_count() {
    let mut count = 0;
    {
        let dc = DropCounter::new(&mut count);
        assert_eq!(dc.count(), 1);
        assert_eq_size_val!(dc, 0usize);
        assert_eq!(dc.count(), 1);
        assert_eq_size_val!(dc, 0usize, dc);
        assert_eq!(dc.count(), 1);
    }
    assert_eq!(count, 0);

    assert_eq_size_val!(DropCounter::new(&mut count), 0usize);
    assert_eq!(count, 0);
}
