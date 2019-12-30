#[derive(Clone, Copy)]
pub struct True;
#[derive(Clone, Copy)]
pub struct False;

impl True {
    pub const fn not(&self) -> &'static False {
        &False
    }
    pub const fn and<'a, T>(&self, other: &'a T) -> &'a T {
        other
    }
    pub const fn or<T>(&self, _: &T) -> &'static True {
        &True
    }
    pub const fn value(&self) -> bool {
        true
    }
}

impl False {
    pub const fn not(&self) -> &'static True {
        &True
    }
    pub const fn and<T>(&self, _: &T) -> &'static False {
        &False
    }
    pub const fn or<'a, T>(&self, other: &'a T) -> &'a T {
        other
    }
    pub const fn value(&self) -> bool {
        false
    }
}

pub trait ToBool: Sized {
    type Bool: Sized;
    const TO_BOOL: Self::Bool;
}

impl ToBool for [(); 0] {
    type Bool = False;
    const TO_BOOL: Self::Bool = False;
}

impl ToBool for [(); 1] {
    type Bool = True;
    const TO_BOOL: Self::Bool = True;
}

/// Converts a `const bool` to a type-level boolean.
#[doc(hidden)]
#[macro_export]
macro_rules! _to_bool {
    ($x:expr) => {{
        const X: bool = $x;
        <[(); X as usize] as $crate::_bool::ToBool>::TO_BOOL
    }};
}
