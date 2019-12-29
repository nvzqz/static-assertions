/// Asserts that types can be passed into external ([FFI]) C code.
///
/// This is useful when ensuring that consumers of library code want to use your
/// public types within an FFI context.
///
/// # Examples
///
/// Rust is used often to work with other languages, especially C. Below are
/// some common examples where different kinds of unsuspecting types cross the
/// FFI boundary.
///
/// ## Opaque Structs
///
/// It's common in an FFI context to operate over a pointer to a type with an
/// unknown size.
///
/// The usual way to make an opaque pointer is by creating a zero-sized type.
/// However, most of the obvious ways of doing this are not FFI-safe!
///
/// - A standalone `struct` does not work:
///
///   ```compile_fail
///   # #[macro_use] extern crate static_assertions;
///   #[repr(C)]
///   struct Opaque;
///
///   assert_ffi_safe!(Opaque);
///   ```
///
/// - Neither a tuple `struct` with an empty tuple inside:
///
///   ```compile_fail
///   # #[macro_use] extern crate static_assertions;
///   #[repr(C)]
///   struct Opaque(());
///
///   assert_ffi_safe!(Opaque);
///   ```
///
/// - Nor a normal `struct` with an empty tuple inside:
///
///   ```compile_fail
///   # #[macro_use] extern crate static_assertions;
///   #[repr(C)]
///   struct Opaque { value: () }
///
///   assert_ffi_safe!(Opaque);
///   ```
///
/// To learn how to do this properly, we must address [The Rustonomicon]!
/// According to the book on the dark arts of unsafe Rust, we should use an
/// empty array field:
///
/// ```
/// # #[macro_use] extern crate static_assertions;
/// #[repr(C)]
/// struct Opaque([u8; 0]);
///
/// assert_ffi_safe!(Opaque);
/// ```
///
/// _Voila!_ And like that, we can safely use a `*const Opaque` in FFI!
///
/// ## Tuples
///
/// Rust tuples do not have a specified layout and thus can't be used in FFI,
/// even when the enclosing type is marked as `#[repr(C)]`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// #[repr(C)]
/// struct Foo {
///     value: (u8, u8)
/// }
///
/// assert_ffi_safe!(Foo);
/// ```
///
/// However, a tuple struct marked as `#[repr(C)]` is FFI-safe:
///
/// ```
/// # #[macro_use] extern crate static_assertions;
/// #[repr(C)]
/// struct Foo(u8, u8);
///
/// assert_ffi_safe!(Foo);
/// ```
///
/// [FFI]: https://en.wikipedia.org/wiki/Foreign_function_interface
/// [The Rustonomicon]: https://doc.rust-lang.org/stable/nomicon/ffi.html#representing-opaque-structs
#[macro_export]
macro_rules! assert_ffi_safe {
    ($($t:ty),+ $(,)?) => {
        $(
            const _: fn() = || {
                extern "C" {
                    #[allow(dead_code)]
                    #[forbid(improper_ctypes)]
                    fn assert_ffi_safe(_: *const $t);
                }
            };
        )+
    }
}
