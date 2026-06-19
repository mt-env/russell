pub mod ast;

pub mod parse_defn;
pub mod parse_expr;
pub mod parse_stmt;
pub mod parse_type;

#[cfg(test)]
mod tests;

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::frontend::error::parse_error::{ParseError, ParseResult};
use crate::frontend::lexer::token::{SpannedToken, Token, TokenKind};
use crate::frontend::parser::ast::ParsedDefn;
use crate::frontend::parser::parse_defn::parse_defn;

pub fn parse<'a>(tokens: Vec<SpannedToken<'a>>) -> Vec<ParsedDefn<'a>> {
    let mut parser = Parser::new(tokens);
    let mut defns = Vec::new();

    while parser.peek().kind() != TokenKind::EoF {
        match parse_defn(&mut parser) {
            Ok(defn) => defns.push(defn),
            Err(_) => todo!(), // TODO - handle errors
        }
    }

    defns
}

pub struct Parser<'a> {
    tokens: Peekable<IntoIter<SpannedToken<'a>>>,
    errors: Vec<ParseError<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<SpannedToken<'a>>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            errors: Vec::new(),
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> ParseResult<'a, ()> {
        if self.peek().kind() == expected {
            self.tokens.next();
            Ok(())
        } else {
            ParseError::new(expected, self.peek())
        }
    }

    pub fn expect_id(&mut self) -> ParseResult<'a, &'a str> {
        match self.take_if(TokenKind::Id) {
            Some(Token::Id(name)) => Ok(name),
            _ => ParseError::new(TokenKind::Id, self.peek()),
        }
    }

    pub fn expect_int(&mut self) -> ParseResult<'a, i64> {
        match self.take_if(TokenKind::Int) {
            Some(Token::Int(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Int, self.peek()),
        }
    }

    pub fn expect_float(&mut self) -> ParseResult<'a, f64> {
        match self.take_if(TokenKind::Float) {
            Some(Token::Float(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Float, self.peek()),
        }
    }

    pub fn expect_bool(&mut self) -> ParseResult<'a, bool> {
        match self.take_if(TokenKind::Bool) {
            Some(Token::Bool(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Bool, self.peek()),
        }
    }

    pub fn expect_typeid(&mut self) -> ParseResult<'a, &'a str> {
        match self.take_if(TokenKind::TypeId) {
            Some(Token::TypeId(name)) => Ok(name),
            _ => ParseError::new(TokenKind::TypeId, self.peek()),
        }
    }

    fn take_if(&mut self, kind: TokenKind) -> Option<Token<'a>> {
        self.tokens.next_if(|t| t.kind() == kind).map(|t| t.node)
    }

    pub fn expect_many(&mut self, expected: &[TokenKind]) -> ParseResult<'a, Token<'a>> {
        for kind in expected {
            if self.peek().kind() == *kind {
                return Ok(self.tokens.next().unwrap().node);
            }
        }

        ParseError::many(expected, self.peek())
    }

    pub fn peek(&mut self) -> &SpannedToken<'a> {
        // EoF sentinel ensures this is always Some
        self.tokens.peek().unwrap()
    }

    // Unconditionally consume and return the next token.
    pub(super) fn advance(&mut self) -> SpannedToken<'a> {
        // EoF sentinel ensures this is always Some
        self.tokens.next().unwrap()
    }

    pub(super) fn push_error(&mut self, error: ParseError<'a>) {
        self.errors.push(error);
    }

    pub(super) fn synchronize(&mut self) {
        // todo - handle edge case
        // there's no rbrace closing a defn - then infinite loop in parse_defn
        // try to find an lbrace - same philosophy as this? but then you need
        // to somehow break out of that loop parse_defn
        while self.peek().kind() != TokenKind::EoF {
            match self.peek().kind() {
                TokenKind::RBrace
                | TokenKind::Let
                | TokenKind::Read
                | TokenKind::Echo
                | TokenKind::Return => return,
                _ => {
                    self.tokens.next();
                }
            }
        }
    }
}
