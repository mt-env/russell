use std::fmt::Display;

use crate::frontend::types::Spanned;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    // primitive values
    Id(&'a str),
    Int(i64),
    Float(f64),
    Bool(bool),

    // keywords
    Echo,
    Else,
    Fn,
    If,
    Let,
    Match,
    Read,
    Return,
    Then,
    Typedef,

    // type keywords
    IntType,
    FloatType,
    BoolType,
    TypeId(&'a str),
    TypeParam(&'a str),

    // punctuation
    LParen,
    RParen,
    Comma,
    Arrow,
    Colon,
    Semicolon,
    LBrace,
    RBrace,

    // operators
    Assign,           // =
    Not,              // !
    NotEq,            // !=
    And,              // &&
    Plus,             // +
    Minus,            // -
    Times,            // *
    Divide,           // /
    FPlus,            // +.
    FMinus,           // -.
    FTimes,           // *.
    FDivide,          // /.
    LessThan,         // <
    LessThanOrEq,     // <=
    GreaterThan,      // >
    GreaterThanOrEq,  // >=
    FLessThan,        // <.
    FLessThanOrEq,    // <=.
    FGreaterThan,     // >.
    FGreaterThanOrEq, // >=.
    Eq,               // ==
    Pipe,             // |>
    Or,               // ||

    // miscellaneous
    EoF,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // primitive values
    Id,
    Int,
    Float,
    Bool,

    // keywords
    Echo,
    Else,
    Fn,
    If,
    Let,
    Match,
    Read,
    Return,
    Then,
    Typedef,

    // type keywords
    IntType,
    FloatType,
    BoolType,
    TypeId,
    TypeParam,

    // punctuation
    LParen,
    RParen,
    Comma,
    Arrow,
    Colon,
    Semicolon,
    LBrace,
    RBrace,

    // operators
    Assign,
    Not,
    NotEq,
    And,
    Plus,
    Minus,
    Times,
    Divide,
    FPlus,
    FMinus,
    FTimes,
    FDivide,
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
    FLessThan,
    FLessThanOrEq,
    FGreaterThan,
    FGreaterThanOrEq,
    Eq,
    Pipe,
    Or,

    // miscellaneous
    EoF,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum LexError<'a> {
    InvalidChar(char),
    InvalidInt(&'a str),
    InvalidFloat(&'a str),
}

pub type SpannedLexError<'a> = Spanned<LexError<'a>>;

impl<'a> SpannedLexError<'a> {
    pub fn new(error: LexError<'a>, offset: usize) -> Self {
        Spanned {
            node: error,
            offset,
        }
    }
}

impl Display for LexError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::InvalidChar(c) => write!(f, "invalid character '{}'", c),
            LexError::InvalidInt(s) => write!(f, "invalid integer literal '{}'", s),
            LexError::InvalidFloat(s) => write!(f, "invalid float literal '{}'", s),
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type SpannedToken<'a> = Spanned<Token<'a>>;
impl Copy for SpannedToken<'_> {}

impl<'a> SpannedToken<'a> {
    pub fn new(token: Token<'a>, offset: usize) -> Self {
        Spanned {
            node: token,
            offset,
        }
    }

    pub fn token(&self) -> &Token<'a> {
        &self.node
    }

    pub fn kind(&self) -> TokenKind {
        self.node.kind()
    }
}

impl Token<'_> {
    pub fn kind(&self) -> TokenKind {
        match self {
            Token::Id(_) => TokenKind::Id,
            Token::Int(_) => TokenKind::Int,
            Token::Float(_) => TokenKind::Float,
            Token::Bool(_) => TokenKind::Bool,
            Token::Echo => TokenKind::Echo,
            Token::Else => TokenKind::Else,
            Token::Fn => TokenKind::Fn,
            Token::If => TokenKind::If,
            Token::Let => TokenKind::Let,
            Token::Match => TokenKind::Match,
            Token::Read => TokenKind::Read,
            Token::Return => TokenKind::Return,
            Token::Then => TokenKind::Then,
            Token::Typedef => TokenKind::Typedef,
            Token::IntType => TokenKind::IntType,
            Token::FloatType => TokenKind::FloatType,
            Token::BoolType => TokenKind::BoolType,
            Token::TypeId(_) => TokenKind::TypeId,
            Token::TypeParam(_) => TokenKind::TypeParam,
            Token::LParen => TokenKind::LParen,
            Token::RParen => TokenKind::RParen,
            Token::Comma => TokenKind::Comma,
            Token::Arrow => TokenKind::Arrow,
            Token::Colon => TokenKind::Colon,
            Token::Semicolon => TokenKind::Semicolon,
            Token::LBrace => TokenKind::LBrace,
            Token::RBrace => TokenKind::RBrace,
            Token::Assign => TokenKind::Assign,
            Token::Not => TokenKind::Not,
            Token::NotEq => TokenKind::NotEq,
            Token::And => TokenKind::And,
            Token::Plus => TokenKind::Plus,
            Token::Minus => TokenKind::Minus,
            Token::Times => TokenKind::Times,
            Token::Divide => TokenKind::Divide,
            Token::FPlus => TokenKind::FPlus,
            Token::FMinus => TokenKind::FMinus,
            Token::FTimes => TokenKind::FTimes,
            Token::FDivide => TokenKind::FDivide,
            Token::LessThan => TokenKind::LessThan,
            Token::LessThanOrEq => TokenKind::LessThanOrEq,
            Token::GreaterThan => TokenKind::GreaterThan,
            Token::GreaterThanOrEq => TokenKind::GreaterThanOrEq,
            Token::FLessThan => TokenKind::FLessThan,
            Token::FLessThanOrEq => TokenKind::FLessThanOrEq,
            Token::FGreaterThan => TokenKind::FGreaterThan,
            Token::FGreaterThanOrEq => TokenKind::FGreaterThanOrEq,
            Token::Eq => TokenKind::Eq,
            Token::Pipe => TokenKind::Pipe,
            Token::Or => TokenKind::Or,
            Token::EoF => TokenKind::EoF,
        }
    }
}
