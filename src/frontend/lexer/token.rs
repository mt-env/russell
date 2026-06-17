use std::fmt::Display;

use crate::frontend::types::Spanned;

#[derive(Debug, Clone, Copy)]
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
    Times,
    Plus,
    Minus,
    Divide,
    LessThan,
    LessThanOrEq,
    Eq,
    GreaterThan,
    GreaterThanOrEq,
    Pipe,
    Or,

    // miscellaneous
    Invalid(char),
    Overflow(&'a str),
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
    Times,
    Plus,
    Minus,
    Divide,
    LessThan,
    LessThanOrEq,
    Eq,
    GreaterThan,
    GreaterThanOrEq,
    Pipe,
    Or,

    // miscellaneous
    Invalid,
    Overflow,
    EoF,
}

impl Display for TokenKind {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type SpannedToken<'a> = Spanned<Token<'a>>;

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
            Token::Times => TokenKind::Times,
            Token::Plus => TokenKind::Plus,
            Token::Minus => TokenKind::Minus,
            Token::Divide => TokenKind::Divide,
            Token::LessThan => TokenKind::LessThan,
            Token::LessThanOrEq => TokenKind::LessThanOrEq,
            Token::Eq => TokenKind::Eq,
            Token::GreaterThan => TokenKind::GreaterThan,
            Token::GreaterThanOrEq => TokenKind::GreaterThanOrEq,
            Token::Pipe => TokenKind::Pipe,
            Token::Or => TokenKind::Or,
            Token::Invalid(_) => TokenKind::Invalid,
            Token::Overflow(_) => TokenKind::Overflow,
            Token::EoF => TokenKind::EoF,
        }
    }
}
