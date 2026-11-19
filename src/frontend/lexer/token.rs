use std::fmt::Display;

use crate::frontend::types::Spanned;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'src> {
    // primitive values
    Id(&'src str),
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
    TypeId(&'src str),
    TypeParam(&'src str),

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
            Self::Id => write!(f, "identifier"),
            Self::Int => write!(f, "integer literal"),
            Self::Float => write!(f, "float literal"),
            Self::Bool => write!(f, "boolean literal"),
            Self::Echo => write!(f, "'echo'"),
            Self::Else => write!(f, "'else'"),
            Self::Fn => write!(f, "'fn'"),
            Self::If => write!(f, "'if'"),
            Self::Let => write!(f, "'let'"),
            Self::Match => write!(f, "'match'"),
            Self::Read => write!(f, "'read'"),
            Self::Return => write!(f, "'return'"),
            Self::Then => write!(f, "'then'"),
            Self::Typedef => write!(f, "'typedef'"),
            Self::IntType => write!(f, "'Int'"),
            Self::FloatType => write!(f, "'Float'"),
            Self::BoolType => write!(f, "'Bool'"),
            Self::TypeId => write!(f, "type identifier"),
            Self::TypeParam => write!(f, "type parameter identifier"),
            Self::LParen => write!(f, "'('"),
            Self::RParen => write!(f, "')'"),
            Self::Comma => write!(f, "','"),
            Self::Arrow => write!(f, "'->'"),
            Self::Colon => write!(f, "':'"),
            Self::Semicolon => write!(f, "';'"),
            Self::LBrace => write!(f, "'{{'"),
            Self::RBrace => write!(f, "'}}'"),
            Self::Assign => write!(f, "'='"),
            Self::Not => write!(f, "'!'"),
            Self::NotEq => write!(f, "'!='"),
            Self::And => write!(f, "'&&'"),
            Self::Plus => write!(f, "'+'"),
            Self::Minus => write!(f, "'-'"),
            Self::Times => write!(f, "'*'"),
            Self::Divide => write!(f, "'/'"),
            Self::FPlus => write!(f, "'+.'"),
            Self::FMinus => write!(f, "'-.'"),
            Self::FTimes => write!(f, "'*.'"),
            Self::FDivide => write!(f, "'/.'"),
            Self::LessThan => write!(f, "'<'"),
            Self::LessThanOrEq => write!(f, "'<='"),
            Self::GreaterThan => write!(f, "'>'"),
            Self::GreaterThanOrEq => write!(f, "'>='"),
            Self::FLessThan => write!(f, "'<.'"),
            Self::FLessThanOrEq => write!(f, "'<=.'"),
            Self::FGreaterThan => write!(f, "'>.'"),
            Self::FGreaterThanOrEq => write!(f, "'>=.'"),
            Self::Eq => write!(f, "'=='"),
            Self::Pipe => write!(f, "'|>'"),
            Self::Or => write!(f, "'||'"),
            Self::EoF => write!(f, "end of file"),
        }
    }
}

pub type SpannedToken<'src> = Spanned<Token<'src>>;

impl<'src> SpannedToken<'src> {
    #[must_use]
    pub const fn new(token: Token<'src>, offset: usize) -> Self {
        Spanned {
            node: token,
            offset,
        }
    }

    #[must_use]
    pub const fn token(&self) -> &Token<'src> {
        &self.node
    }

    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.node.kind()
    }
}

impl Token<'_> {
    #[must_use]
    pub const fn kind(&self) -> TokenKind {
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
