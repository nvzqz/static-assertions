//! Parse assertion attributes.
//!
//! Context-free grammar:
//!
//! ```txt
//! Attr = List<Assert>
//!
//! List<T> = T
//!         | T ,
//!         | T , List<T>
//!
//! Assert = Cond
//!        | impl_all ( List<Trait> )
//!        | impl_any ( List<Trait> )
//!        | ! impl_all ( List<Trait> )
//!        | ! impl_any ( List<Trait> )
//!
//! Trait = Code
//!
//! Code = <&str>
//!
//! Cond = <bool>
//!      | ( Cond )
//!      | ! Cond
//!      | MemProp CmpOp <usize>
//!      | Cond BinOp Cond
//!
//! MemProp = 'size'
//!         | 'align'
//!
//! BinOp = CmpOp
//!       | '&&'
//!       | '||'
//!
//! CmpOp = '=='
//!       | '!='
//!       | '<'
//!       | '<='
//!       | '>'
//!       | '>='
//! ```

use std::num::ParseIntError;

mod generic;
mod op;

pub use self::{
    generic::*,
    op::*,
};

/// A type that can be parsed from a string slice.
///
/// When implementing this trait, it's important that the implementer is not
/// expected to call [`str::trim_start`]. When calling `parse`, one should pass
/// a pre-trimmed string. This is done for performance.
///
/// [`str::trim_start`]: https://doc.rust-lang.org/std/primitive.str.html#method.trim_start
pub trait Parse<'a>: Sized + 'a {
    /// Attempts to parse the beginning of `input`, consuming it. Returns the
    /// parsed value `Self` and the unconsumed remainder of `s`, or an error if
    /// `Self` can't be parsed from `input`.
    ///
    /// `input` is expected to already have the start trimmed. This improves
    /// performance when attempting to parse various syntax options.
    fn parse(input: &'a str) -> Result<(Self, &'a str), ParseError>;

    #[cfg(test)]
    fn parse_eq(&self, s: &'a str) -> bool where Self: PartialEq {
        Self::parse(s).as_ref().map(|(p, _)| p) == Ok(self)
    }
}

/// The error returned when `Parse::parse` fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// The string is empty. This includes whitespace-only strings.
    Empty,
    /// Failed to parse a memory property (e.g. `size`, `align`).
    MemProp,
    /// Failed to parse a comparison operation.
    CmpOp,
    /// Failed to parse a boolean condition.
    Cond,
    ///
    Paren,
    ///
    Bool,
    /// Failed to parse an integer.
    Int(Option<ParseIntError>),
}

impl From<ParseIntError> for ParseError {
    #[inline]
    fn from(error: ParseIntError) -> Self {
        ParseError::Int(Some(error))
    }
}

/// Parses the expression `$s` by consuming `$pattern` and returns `Some($val)`
/// on success.
macro_rules! parse_ident {
    ($s:expr, $error:ident: $($pattern:expr => $val:expr,)+) => {
        $(if $s.starts_with($pattern) {
            Ok(($val, &$s[$pattern.len()..]))
        } else)+ {
            Err(ParseError::$error)
        }
    };
}

// Empty value
impl<'a> Parse<'a> for () {
    #[inline]
    fn parse(s: &'a str) -> Result<(Self, &'a str), ParseError> {
        Ok(((), s))
    }
}

/// An "attribute"; actually the input that goes inside `#[assert(...)]`.
///
/// Context-free grammar:
///
/// ```txt
/// Attr = List<Assert>
/// ```
pub type Attr<'a> = List<Assert<'a>>;

/// An assertion.
pub enum Assert<'a> {
    Cond(Cond),
    ImplAll(List<Trait<'a>>),
    ImplAny(List<Trait<'a>>),
    NotImplAll(List<Trait<'a>>),
    NotImplAny(List<Trait<'a>>),
}

pub struct Trait<'a>(&'a str);

/// A memory property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemProp {
    /// `size`.
    Size,
    /// `align`.
    Align,
}

impl<'a> Parse<'a> for MemProp {
    fn parse(s: &'a str) -> Result<(Self, &'a str), ParseError> {
        parse_ident! {
            s, MemProp:
            "size"  => MemProp::Size,
            "align" => MemProp::Align,
        }
    }
}

/// A boolean condition.
///
/// Context-free grammar:
///
/// ```txt
/// Cond = <bool>
///      | ( Cond )
///      | ! Cond
///      | MemProp CmpOp <usize>
///      | Cond BinOp Cond
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cond {
    /// A boolean literal; `true` or `false`.
    Bool(bool),
    /// A memory property comparison.
    MemPropCmp(MemProp, CmpOp, usize),
    ///
    Paren(Box<Cond>),
    ///
    Neg(Box<Cond>),
    ///
    BinOp(Box<Cond>, BinOp, Box<Cond>),
}

impl<'a> Parse<'a> for bool {
    fn parse(s: &'a str) -> Result<(Self, &'a str), ParseError> {
        parse_ident! {
            s, Bool:
            "true"  => true,
            "false" => false,
        }
    }
}

impl<'a> Parse<'a> for usize {
    fn parse(s: &'a str) -> Result<(Self, &'a str), ParseError> {
        s.as_bytes()
            .iter()
            .enumerate()
            .find(|(_, &ch)| match ch {
                b'0'..=b'9' => false,
                _ => true,
            })
            .ok_or(ParseError::Int(None))
            .and_then(|(last, _)| {
                let (start, end) = s.split_at(last);
                Ok((start.parse::<Self>()?, end))
            })
    }
}

impl<'a> Parse<'a> for Cond {
    fn parse(s: &'a str) -> Result<(Self, &'a str), ParseError> {
        if let Ok((parsed, rest)) = bool::parse(s) {
            return Ok((Cond::Bool(parsed), rest));
        }

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize() {
        let mut values: Vec<usize> = (0..=100).collect();
        values.push(usize::max_value());

        for value in values {
            for input in [
                format!("{}", value),
                format!("{} ", value),
                format!("{})", value),
                format!("{}abcd", value), // TODO: Should this be accepted?
            ].iter() {

            }
        }
    }

    #[test]
    fn mem_prop() {
        use self::MemProp::*;

        let props = [
            ("size",   "",  Size),
            ("size ",  " ", Size),
            ("align",  "",  Align),
            ("align ", " ", Align),
        ];

        for &(input, remainder, parsed) in props.iter() {
            let output = MemProp::parse(input.trim_start());
            assert_eq!(output, Ok((parsed, remainder)));
        }
    }
}
