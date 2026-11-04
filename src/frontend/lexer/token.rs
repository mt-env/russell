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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Id => write!(f, "identifier"),
            TokenKind::Int => write!(f, "integer literal"),
            TokenKind::Float => write!(f, "float literal"),
            TokenKind::Bool => write!(f, "boolean literal"),
            TokenKind::Echo => write!(f, "'echo'"),
            TokenKind::Else => write!(f, "'else'"),
            TokenKind::Fn => write!(f, "'fn'"),
            TokenKind::If => write!(f, "'if'"),
            TokenKind::Let => write!(f, "'let'"),
            TokenKind::Match => write!(f, "'match'"),
            TokenKind::Read => write!(f, "'read'"),
            TokenKind::Return => write!(f, "'return'"),
            TokenKind::Then => write!(f, "'then'"),
            TokenKind::Typedef => write!(f, "'typedef'"),
            TokenKind::IntType => write!(f, "'Int'"),
            TokenKind::FloatType => write!(f, "'Float'"),
            TokenKind::BoolType => write!(f, "'Bool'"),
            TokenKind::TypeId => write!(f, "type identifier"),
            TokenKind::TypeParam => write!(f, "type parameter identifier"),
            TokenKind::LParen => write!(f, "'('"),
            TokenKind::RParen => write!(f, "')'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Arrow => write!(f, "'->'"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::Semicolon => write!(f, "';'"),
            TokenKind::LBrace => write!(f, "'{{'"),
            TokenKind::RBrace => write!(f, "'}}'"),
            TokenKind::Assign => write!(f, "'='"),
            TokenKind::Not => write!(f, "'!'"),
            TokenKind::NotEq => write!(f, "'!='"),
            TokenKind::And => write!(f, "'&&'"),
            TokenKind::Plus => write!(f, "'+'"),
            TokenKind::Minus => write!(f, "'-'"),
            TokenKind::Times => write!(f, "'*'"),
            TokenKind::Divide => write!(f, "'/'"),
            TokenKind::FPlus => write!(f, "'+.'"),
            TokenKind::FMinus => write!(f, "'-.'"),
            TokenKind::FTimes => write!(f, "'*.'"),
            TokenKind::FDivide => write!(f, "'/.'"),
            TokenKind::LessThan => write!(f, "'<'"),
            TokenKind::LessThanOrEq => write!(f, "'<='"),
            TokenKind::GreaterThan => write!(f, "'>'"),
            TokenKind::GreaterThanOrEq => write!(f, "'>='"),
            TokenKind::FLessThan => write!(f, "'<.'"),
            TokenKind::FLessThanOrEq => write!(f, "'<=.'"),
            TokenKind::FGreaterThan => write!(f, "'>.'"),
            TokenKind::FGreaterThanOrEq => write!(f, "'>=.'"),
            TokenKind::Eq => write!(f, "'=='"),
            TokenKind::Pipe => write!(f, "'|>'"),
            TokenKind::Or => write!(f, "'||'"),
            TokenKind::EoF => write!(f, "end of file"),
        }
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
