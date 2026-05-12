pub mod ast;

mod parse_defn;
mod parse_expr;
mod parse_stmt;
mod parse_type;

#[cfg(test)]
mod tests;

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::frontend::error::parse_error::{ParseError, ParseResult};
use crate::frontend::lexer::token::{SpannedToken, Token, TokenKind};
use crate::frontend::parser::ast::ParsedDefn;
use crate::frontend::parser::parse_defn::parse_defn;

pub fn parse(tokens: Vec<SpannedToken>) -> Vec<ParsedDefn> {
    let mut parser = Parser::new(tokens);
    let mut defns = Vec::new();

    while parser.peek().kind() != TokenKind::EoF {
        match parse_defn(&mut parser) {
            Ok(defn) => defns.push(defn),
            Err(err) => unimplemented!(), // TODO - handle errors
        }
    }

    defns
}

pub struct Parser {
    tokens: Peekable<IntoIter<SpannedToken>>,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> ParseResult<()> {
        if self.peek().kind() == expected {
            self.tokens.next();
            Ok(())
        } else {
            ParseError::new(expected, self.peek())
        }
    }

    pub fn expect_id(&mut self) -> ParseResult<String> {
        match self.take_if(TokenKind::Id) {
            Some(Token::Id(name)) => Ok(name),
            _ => ParseError::new(TokenKind::Id, self.peek()),
        }
    }

    pub fn expect_int(&mut self) -> ParseResult<i64> {
        match self.take_if(TokenKind::Int) {
            Some(Token::Int(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Int, self.peek()),
        }
    }

    pub fn expect_float(&mut self) -> ParseResult<f64> {
        match self.take_if(TokenKind::Float) {
            Some(Token::Float(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Float, self.peek()),
        }
    }

    pub fn expect_bool(&mut self) -> ParseResult<bool> {
        match self.take_if(TokenKind::Bool) {
            Some(Token::Bool(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Bool, self.peek()),
        }
    }

    pub fn expect_typeid(&mut self) -> ParseResult<String> {
        match self.take_if(TokenKind::TypeId) {
            Some(Token::TypeId(name)) => Ok(name),
            _ => ParseError::new(TokenKind::TypeId, self.peek()),
        }
    }

    fn take_if(&mut self, kind: TokenKind) -> Option<Token> {
        self.tokens.next_if(|t| t.kind() == kind).map(|t| t.token)
    }

    pub fn expect_many(&mut self, expected: &[TokenKind]) -> ParseResult<Token> {
        for kind in expected {
            if self.peek().kind() == *kind {
                return Ok(self.tokens.next().unwrap().token);
            }
        }

        ParseError::many(expected, self.peek())
    }

    pub fn peek(&mut self) -> &SpannedToken {
        // EoF sentinel ensures this is always Some
        self.tokens.peek().unwrap()
    }

    // Unconditionally consume and return the next token.
    pub(super) fn advance(&mut self) -> SpannedToken {
        // EoF sentinel ensures this is always Some
        self.tokens.next().unwrap()
    }
}

