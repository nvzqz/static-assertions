#[derive(Clone, Copy)]
pub struct True;
#[derive(Clone, Copy)]
pub struct False;

impl True {
    pub fn not(self) -> False { False }
    pub fn and<T>(self, other: T) -> T { other }
    pub fn or<T>(self, _: T) -> True { True }
}

impl False {
    pub fn not(self) -> True { True }
    pub fn and<T>(self, _: T) -> False { False }
    pub fn or<T>(self, other: T) -> T { other }
}

pub trait ToBool {
    type Value;
    fn to_bool(_: Self) -> Self::Value;
}

impl ToBool for [(); 0] {
    type Value = False;
    fn to_bool(_: Self) -> Self::Value {
        False
    }
}

impl ToBool for [(); 1] {
    type Value = True;
    fn to_bool(_: Self) -> Self::Value {
        True
    }
}

/// Converts a `const bool` to a type-level boolean.
#[doc(hidden)]
#[macro_export]
macro_rules! to_bool {
    ($x:expr) => {{
        const ASSERT: bool = $x;
        $crate::type_level_bool::ToBool::to_bool([(); ASSERT as usize])
    }};
}
