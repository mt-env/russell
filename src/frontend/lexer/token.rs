use std::fmt::Display;

#[derive(Debug, Clone)]
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
    TypeId(String),

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
    EoF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_print = match &self {
            TokenKind::Id => todo!(),
            TokenKind::Int => todo!(),
            TokenKind::Float => todo!(),
            TokenKind::Bool => todo!(),
            TokenKind::Echo => todo!(),
            TokenKind::Else => todo!(),
            TokenKind::Fn => todo!(),
            TokenKind::If => todo!(),
            TokenKind::Let => todo!(),
            TokenKind::Match => todo!(),
            TokenKind::Read => todo!(),
            TokenKind::Return => todo!(),
            TokenKind::Then => todo!(),
            TokenKind::Typedef => todo!(),
            TokenKind::IntType => todo!(),
            TokenKind::FloatType => todo!(),
            TokenKind::BoolType => todo!(),
            TokenKind::TypeId => todo!(),
            TokenKind::LParen => todo!(),
            TokenKind::RParen => todo!(),
            TokenKind::Comma => todo!(),
            TokenKind::Arrow => todo!(),
            TokenKind::Colon => todo!(),
            TokenKind::Semicolon => todo!(),
            TokenKind::LBrace => todo!(),
            TokenKind::RBrace => todo!(),
            TokenKind::Assign => todo!(),
            TokenKind::Not => todo!(),
            TokenKind::NotEq => todo!(),
            TokenKind::And => todo!(),
            TokenKind::Times => todo!(),
            TokenKind::Plus => todo!(),
            TokenKind::Minus => todo!(),
            TokenKind::Divide => todo!(),
            TokenKind::LessThan => todo!(),
            TokenKind::LessThanOrEq => todo!(),
            TokenKind::Eq => todo!(),
            TokenKind::GreaterThan => todo!(),
            TokenKind::GreaterThanOrEq => todo!(),
            TokenKind::Pipe => todo!(),
            TokenKind::Or => todo!(),
            TokenKind::Invalid => todo!(),
            TokenKind::EoF => todo!(),
        };
        write!(f, "{}", pretty_print)
    }
}

#[derive(Debug, Clone)]
pub struct SpannedToken<'a> {
    pub token: Token<'a>,
    pub offset: usize,
}

impl SpannedToken<'_> {
    pub fn kind(&self) -> TokenKind {
        self.token.kind()
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
            Token::EoF => TokenKind::EoF,
        }
    }
}
