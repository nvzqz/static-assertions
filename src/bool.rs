#[derive(Clone, Copy)]
pub struct True;
#[derive(Clone, Copy)]
pub struct False;

impl True {
    pub const fn not<'a>(&'a self) -> &'a False {
        &False
    }
    pub const fn and<'a, T>(&'a self, other: &'a T) -> &'a T {
        other
    }
    pub const fn or<'a, T>(&'a self, _: &'a T) -> &'a True {
        &True
    }
}

impl False {
    pub const fn not<'a>(&'a self) -> &'a True {
        &True
    }
    pub const fn and<'a, T>(&'a self, _: &'a T) -> &'a False {
        &False
    }
    pub const fn or<'a, T>(&'a self, other: &'a T) -> &'a T {
        other
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
