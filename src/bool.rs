use core::ops::{BitAnd, BitOr, BitXor, Not};

pub struct True;

impl BitAnd<True> for True {
    type Output = True;

    fn bitand(self, _: True) -> True {
        True
    }
}

impl BitAnd<False> for True {
    type Output = False;

    fn bitand(self, _: False) -> False {
        False
    }
}

impl<T> BitOr<T> for True {
    type Output = True;

    fn bitor(self, _: T) -> True {
        True
    }
}

impl BitXor<True> for True {
    type Output = False;

    fn bitxor(self, _: True) -> False {
        False
    }
}

impl BitXor<False> for True {
    type Output = True;

    fn bitxor(self, _: False) -> True {
        True
    }
}

impl Not for True {
    type Output = False;

    fn not(self) -> False {
        False
    }
}

pub struct False;

impl<T> BitAnd<T> for False {
    type Output = False;

    fn bitand(self, _: T) -> False {
        False
    }
}

impl BitOr<True> for False {
    type Output = True;

    fn bitor(self, _: True) -> True {
        True
    }
}

impl BitOr<False> for False {
    type Output = False;

    fn bitor(self, _: False) -> False {
        False
    }
}

impl BitXor<False> for False {
    type Output = True;

    fn bitxor(self, _: False) -> True {
        True
    }
}

impl BitXor<True> for False {
    type Output = False;

    fn bitxor(self, _: True) -> False {
        False
    }
}

impl Not for False {
    type Output = True;

    fn not(self) -> True {
        True
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
