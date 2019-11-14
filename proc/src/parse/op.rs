use std::{fmt, str};
use super::{Parse, ParseError};

/// A binary operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinOp {
    CmpOp(CmpOp),
}

impl<'a> Parse<'a> for BinOp {
    fn parse(s: &'a str) -> Result<(Self, &'a str), ParseError> {
        CmpOp::parse(s)
            .map(|(op, rest)| (BinOp::CmpOp(op), rest))
    }
}

/// Comparison operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmpOp {
    /// `==`.
    Eq,
    /// `!=`.
    Ne,
    /// `<`.
    Lt,
    /// `<=`.
    LtEq,
    /// `>`.
    Gt,
    /// `>=`.
    GtEq,
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for CmpOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(CmpOp::Eq),
            "!=" => Ok(CmpOp::Ne),
            "<"  => Ok(CmpOp::Lt),
            "<=" => Ok(CmpOp::LtEq),
            ">"  => Ok(CmpOp::Gt),
            ">=" => Ok(CmpOp::GtEq),
            _ => Err(()),
        }
    }
}

impl<'a> Parse<'a> for CmpOp {
    fn parse(s: &'a str) -> Result<(Self, &'a str), ParseError> {
        macro_rules! parse {
            ($($pat:expr => $op:ident,)+) => {
                $(if s.starts_with($pat) {
                    Ok((CmpOp::$op, &s[$pat.len()..]))
                } else)+ {
                    Err(ParseError::CmpOp)
                }
            };
        }
        parse! {
            "==" => Eq,
            "!=" => Ne,
            "<"  => Lt,
            "<=" => LtEq,
            ">"  => Gt,
            ">=" => GtEq,
        }
    }
}

impl CmpOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            CmpOp::Eq   => "==",
            CmpOp::Ne   => "!=",
            CmpOp::Lt   => "<" ,
            CmpOp::LtEq => "<=",
            CmpOp::Gt   => ">" ,
            CmpOp::GtEq => ">=",
        }
    }
}
