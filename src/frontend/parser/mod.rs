pub(crate) mod ast;

pub(crate) mod parse_defn;
pub(crate) mod parse_expr;
pub(crate) mod parse_stmt;
pub(crate) mod parse_type;

#[cfg(test)]
mod tests;

use std::{iter::Peekable, vec::IntoIter};

use crate::frontend::{
    error::parse_error::{ParseError, ParseResult},
    lexer::token::{SpannedToken, Token, TokenKind},
    parser::{ast::ParsedDefn, parse_defn::parse_defn},
};

pub fn parse<'src>(tokens: Vec<SpannedToken<'src>>) -> Vec<ParsedDefn<'src>> {
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

pub(crate) struct Parser<'src> {
    tokens: Peekable<IntoIter<SpannedToken<'src>>>,
    errors: Vec<ParseError<'src>>,
}

impl<'src> Parser<'src> {
    pub(crate) fn new(tokens: Vec<SpannedToken<'src>>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            errors: Vec::new(),
        }
    }

    fn expect(&mut self, expected: TokenKind) -> ParseResult<'src, ()> {
        if self.peek().kind() == expected {
            self.tokens.next();
            Ok(())
        } else {
            ParseError::new(expected, self.peek())
        }
    }

    fn expect_id(&mut self) -> ParseResult<'src, &'src str> {
        match self.take_if(TokenKind::Id) {
            Some(Token::Id(name)) => Ok(name),
            _ => ParseError::new(TokenKind::Id, self.peek()),
        }
    }

    fn expect_int(&mut self) -> ParseResult<'src, i64> {
        match self.take_if(TokenKind::Int) {
            Some(Token::Int(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Int, self.peek()),
        }
    }

    fn expect_float(&mut self) -> ParseResult<'src, f64> {
        match self.take_if(TokenKind::Float) {
            Some(Token::Float(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Float, self.peek()),
        }
    }

    fn expect_bool(&mut self) -> ParseResult<'src, bool> {
        match self.take_if(TokenKind::Bool) {
            Some(Token::Bool(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Bool, self.peek()),
        }
    }

    fn expect_typeid(&mut self) -> ParseResult<'src, &'src str> {
        match self.take_if(TokenKind::TypeId) {
            Some(Token::TypeId(name)) => Ok(name),
            _ => ParseError::new(TokenKind::TypeId, self.peek()),
        }
    }

    fn expect_typeparam(&mut self) -> ParseResult<'src, &'src str> {
        match self.take_if(TokenKind::TypeParam) {
            Some(Token::TypeParam(name)) => Ok(name),
            _ => ParseError::new(TokenKind::TypeParam, self.peek()),
        }
    }

    fn take_if(&mut self, kind: TokenKind) -> Option<Token<'src>> {
        self.tokens.next_if(|t| t.kind() == kind).map(|t| t.node)
    }

    fn expect_many(&mut self, expected: &[TokenKind]) -> ParseResult<'src, Token<'src>> {
        for kind in expected {
            if self.peek().kind() == *kind {
                return Ok(self.tokens.next().unwrap().node);
            }
        }

        ParseError::many(expected, self.peek())
    }

    pub fn peek(&mut self) -> &SpannedToken<'src> {
        // EoF sentinel ensures this is always Some
        self.tokens.peek().unwrap()
    }

    // Unconditionally consume and return the next token.
    pub(super) fn advance(&mut self) -> SpannedToken<'src> {
        // EoF sentinel ensures this is always Some
        self.tokens.next().unwrap()
    }
}
