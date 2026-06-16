use std::fmt;

use crate::frontend::lexer::token::{SpannedToken, Token};

#[derive(Debug)]
pub struct LexError<'a> {
    pub kind: LexErrorKind<'a>,
    pub offset: usize,
}

#[derive(Debug)]
pub enum LexErrorKind<'a> {
    InvalidCharacter(char),
    IntOverflow(&'a str),
}

impl fmt::Display for LexError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LexErrorKind::InvalidCharacter(c) => write!(f, "invalid character '{}'", c),
            LexErrorKind::IntOverflow(str) => write!(f, "integer literal overflow {}", str),
        }
    }
}

impl std::error::Error for LexError<'_> {}

pub fn collect_errors<'a>(tokens: &[SpannedToken<'a>]) -> Vec<LexError<'a>> {
    tokens
        .iter()
        .filter_map(|t| match &t.token() {
            Token::Invalid(c) => Some(LexError {
                kind: LexErrorKind::InvalidCharacter(*c),
                offset: t.offset,
            }),
            Token::Overflow(str) => Some(LexError {
                kind: LexErrorKind::IntOverflow(str),
                offset: t.offset,
            }),
            _ => None,
        })
        .collect()
}
