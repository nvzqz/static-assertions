#[macro_use]
extern crate proc_static_assertions;

#[assert(size == 4, align == 4)]
struct Foo {
    value: i32,
}
