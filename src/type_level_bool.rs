#[derive(Clone, Copy)]
pub struct True;
#[derive(Clone, Copy)]
pub struct False;

const TRUE: True = True;
const FALSE: False = False;

impl True {
    pub const fn not<'a>(&'a self) -> &'a False { &FALSE }
    pub const fn and<'a, T>(&'a self, other: &'a T) -> &'a T { other }
    pub const fn or<'a, T>(&'a self, _: &'a T) -> &'a True { &TRUE }
}

impl False {
    pub const fn not<'a>(&'a self) -> &'a True { &TRUE }
    pub const fn and<'a, T>(&'a self, _: &'a T) -> &'a False { &FALSE }
    pub const fn or<'a, T>(&'a self, other: &'a T) -> &'a T { other }
}

pub trait ToBool: Sized {
    type Value: Sized;
    const TO_BOOL: Self::Value;
}

impl ToBool for [(); 0] {
    type Value = False;
    const TO_BOOL: Self::Value = False;
}

impl ToBool for [(); 1] {
    type Value = True;
    const TO_BOOL: Self::Value = True;
}

/// Converts a `const bool` to a type-level boolean.
#[doc(hidden)]
#[macro_export]
macro_rules! to_bool {
    ($x:expr) => {
        <[(); $x as usize] as $crate::type_level_bool::ToBool>::TO_BOOL
    };
}
