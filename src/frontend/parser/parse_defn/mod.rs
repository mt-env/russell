use crate::frontend::lexer::token::TokenKind;
use crate::frontend::parser::ast::{Binding, Defn, ParsedDefn};
use crate::frontend::parser::parse_stmt::parse_stmnt;
use crate::frontend::parser::parse_type::{parse_binding_list, parse_type};
use crate::frontend::error::parse_error::{ParseError, ParseResult};
use crate::frontend::parser::Parser;

#[cfg(test)]
mod tests;

pub(super) fn parse_defn(parser: &mut Parser) -> ParseResult<ParsedDefn> {
    match parser.peek().kind() {
        TokenKind::Fn => parse_fndef(parser),
        TokenKind::Typedef => parse_typedef(parser),
        _ => ParseError::many(&[TokenKind::Fn, TokenKind::Typedef], parser.peek()),
    }
}

/// Parses a type definition:
/// typedef <typeId> { <id>(<binding, ...), ... }
fn parse_typedef(parser: &mut Parser) -> ParseResult<ParsedDefn> {
    // parse the declaration
    parser.expect(TokenKind::Typedef)?;
    let name = parser.expect_typeid()?;
    parser.expect(TokenKind::LBrace)?;

    // parse all product types in the ADT
    let mut signatures = Vec::new();
    if parser.peek().kind() != TokenKind::RBrace {
        signatures.push(parse_fn_sig(parser)?);
        while parser.peek().kind() == TokenKind::Comma {
            parser.advance();
            signatures.push(parse_fn_sig(parser)?);
        }
    }

    parser.expect(TokenKind::RBrace)?;

    Ok(Defn::Typedef(name, signatures))
}

/// Parses a function definition:
/// fn <id>(<binding>, ...) -> <type> { <stmnt>, ... };
fn parse_fndef(parser: &mut Parser) -> ParseResult<ParsedDefn> {
    // parse the function header (identifier, bindings, return type)
    parser.expect(TokenKind::Fn)?;
    let header = parse_fn_sig(parser)?;
    parser.expect(TokenKind::Arrow)?;
    let return_type = parse_type(parser)?;

    // parse the function body (LBrace, statements, RBrace, Semicolon)
    parser.expect(TokenKind::LBrace)?;

    let mut statements = Vec::new();
    while parser.peek().kind() != TokenKind::RBrace {
        match parse_stmnt(parser) {
            Ok(stmnt) => statements.push(stmnt),
            Err(_) => unimplemented!(), // TODO improve error handling
        }
    }

    parser.expect(TokenKind::RBrace)?;

    Ok(Defn::Fn(header.0, header.1, return_type, statements))
}

/// Parse a function signature: <id>(<binding>, ...)
/// Returns the ID and a list of bindings if successful.
/// Returns an error otherwise.
fn parse_fn_sig(parser: &mut Parser) -> ParseResult<(String, Vec<Binding>)> {
    let id = parser.expect_id()?;
    let bindings = parse_binding_list(parser)?;
    Ok((id, bindings))
}
