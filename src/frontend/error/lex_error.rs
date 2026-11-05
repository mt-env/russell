use crate::frontend::{error::CompilerError, types::Spanned};

pub type LexError<'src> = Spanned<LexErrorKind<'src>>;

#[derive(Debug, PartialEq)]
pub enum LexErrorKind<'src> {
    InvalidChar(char),
    InvalidInt(&'src str),
    InvalidFloat(&'src str),
}

impl<'src> LexError<'src> {
    pub fn new(error: LexErrorKind<'src>, offset: usize) -> Self {
        Spanned {
            node: error,
            offset,
        }
    }
}

impl<'src> CompilerError<'src> for LexError<'src> {
    fn offset(&self) -> usize {
        self.offset
    }
}

impl<'src> std::fmt::Display for LexError<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.node {
            LexErrorKind::InvalidChar(c) => write!(f, "invalid character '{}'", c),
            LexErrorKind::InvalidInt(s) => write!(f, "invalid integer '{}'", s),
            LexErrorKind::InvalidFloat(s) => write!(f, "invalid float '{}'", s),
        }
    }
}
