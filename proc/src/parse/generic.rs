use super::{Parse, ParseError};

/// A comma-separated list of parsable items.
///
/// Context-free grammar:
///
/// ```txt
/// List<T> = T
///         | T ,
///         | T , List<T>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct List<T>(pub Vec<T>);

impl<'a, T: Parse<'a>> Parse<'a> for List<T> {
    fn parse(mut s: &'a str) -> Result<(Self, &'a str), ParseError> {
        let mut list = Vec::<T>::new();
        loop {
            match T::parse(s) {
                Ok((parsed, rest)) => {
                    list.push(parsed);
                    s = rest;
                },
                Err(ParseError::Empty) if !list.is_empty() => {
                    break Ok((List(list), ""));
                },
                Err(error) => {
                    break Err(error);
                },
            }
        }
    }
}

/// An item surrounded by parentheses.
///
/// Context-free grammar:
///
/// ```txt
/// Paren<T> = ( T )
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Paren<T>(pub T);

impl<'a, T: Parse<'a>> Parse<'a> for Paren<T> {
    fn parse(mut s: &'a str) -> Result<(Self, &'a str), ParseError> {
        // Open parenthesis
        if !s.starts_with("(") {
            return Err(ParseError::Paren);
        } else {
            s = &s[1..].trim_start();
        }

        // Inner item
        let (inner, rest) = T::parse(s)?;
        s = rest.trim_start();

        // Close parenthesis
        if !s.starts_with(")") {
            return Err(ParseError::Paren);
        } else {
            s = &s[1..];
        }

        Ok((Paren(inner), s))
    }
}
