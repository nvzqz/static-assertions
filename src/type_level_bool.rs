pub struct True;
pub struct False;

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
