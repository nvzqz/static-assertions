/// Asserts that the type has the given field offsets.
///
/// Due to implementation details, this only works for [`Sized`] types.
///
/// # Safety
///
/// This is implemented by creating a dangling reference to an uninitialized
/// memory. Currently, this works on supported Rust versions (as of this
/// writing: 1.37 and 1.38). However, a test that is expected to compile today
/// may not in the future.
///
/// # Examples
///
/// Given some type whose fields can't be re-ordered because of `#[repr(C)]`:
///
/// ```
/// #[repr(C)]
/// struct Foo {
///     start: u32,
///     end:   u64,
/// }
/// ```
///
/// The following example fails to compile because, despite `start` being 4
/// bytes, `end` is aligned to 8 bytes and thus requires an offset of 8.
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # #[repr(C)] struct Foo { start: u32, end: u64 }
/// assert_field_offsets!(Foo { end: == 4 });
/// ```
///
/// However, the correct assumptions are upheld below:
///
/// ```
/// # #[macro_use] extern crate static_assertions;
/// # #[repr(C)] struct Foo { start: u32, end: u64 }
/// assert_field_offsets!(Foo {
///     start: == 0,
///     end:   == 8,
/// });
/// ```
///
/// This macro even works with tuple `struct`s:
///
/// ```
/// # #[macro_use] extern crate static_assertions;
/// struct Bar(u32, u32);
///
/// assert_field_offsets!(Bar { 1: == 4 });
/// ```
///
/// [`Sized`]: https://doc.rust-lang.org/std/marker/trait.Sized.html
#[macro_export]
macro_rules! assert_field_offsets {
    ($t:ty { $($f:tt: == $o:expr),+ $(,)? }) => {
        $(
            const _: [(); $o] = [(); unsafe {
                // FIXME: Creating a reference to uninitialized memory is
                // undefined behavior, but does that apply at compile time or
                // only at run time?
                // TODO: Make this work with unsized types
                union Cast<T: 'static> {
                    ptr: &'static T,
                    addr: usize,
                }

                let align = $crate::_core::mem::align_of::<$t>();
                let ptr: &$t = Cast { addr: align }.ptr;

                Cast { ptr: &ptr.$f }.addr - align
            }];
        )+
    };
}
