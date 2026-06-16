use std::fmt;

use crate::frontend::lexer::token::{SpannedToken, Token, TokenKind};

#[derive(Debug)]
pub struct ParseError<'a> {
    pub expected: Vec<TokenKind>,
    pub actual: Token<'a>,
    pub offset: usize,
}

impl<'a> ParseError<'a> {
    pub fn new<A>(expected: TokenKind, actual: &SpannedToken<'a>) -> ParseResult<'a, A> {
        Err(ParseError {
            expected: vec![expected],
            actual: *actual.token(),
            offset: actual.offset,
        })
    }

    pub fn many<A>(expected: &[TokenKind], actual: &SpannedToken<'a>) -> ParseResult<'a, A> {
        Err(ParseError {
            expected: expected.to_vec(),
            actual: *actual.token(),
            offset: actual.offset,
        })
    }
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.expected.as_slice() {
            [single] => write!(f, "expected {}, found {}", single, self.actual.kind()),
            many => {
                write!(f, "expected one of ")?;
                for (i, kind) in many.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", kind)?;
                }
                write!(f, "; found {}", self.actual.kind())
            }
        }
    }
}

impl std::error::Error for ParseError<'_> {}

pub type ParseResult<'a, A> = Result<A, ParseError<'a>>;
