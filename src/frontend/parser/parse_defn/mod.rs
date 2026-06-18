use crate::frontend::error::parse_error::{ParseError, ParseResult};
use crate::frontend::lexer::token::TokenKind;
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::{ParsedDefn, SpannedBinding};
use crate::frontend::parser::parse_stmt::parse_stmnt;
use crate::frontend::parser::parse_type::{parse_binding_list, parse_type};

#[cfg(test)]
mod tests;

pub(super) fn parse_defn<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedDefn<'a>> {
    match parser.peek().kind() {
        TokenKind::Fn => parse_fndef(parser),
        TokenKind::Typedef => parse_typedef(parser),
        _ => ParseError::many(&[TokenKind::Fn, TokenKind::Typedef], parser.peek()),
    }
}

/// Parses a type definition:
/// typedef <typeId> { <id>(<binding, ...), ... }
fn parse_typedef<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedDefn<'a>> {
    let loc = parser.peek().offset;

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

    Ok(ParsedDefn::make_typedef(loc, name, signatures))
}

/// Parses a function definition:
/// fn <id>(<binding>, ...) -> <type> { <stmnt>, ... };
fn parse_fndef<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedDefn<'a>> {
    let loc = parser.peek().offset;

    // parse the function header (identifier, bindings, return type)
    parser.expect(TokenKind::Fn)?;
    let header = parse_fn_sig(parser)?;
    parser.expect(TokenKind::Arrow)?;
    let return_type = parse_type(parser)?;

    // parse the function body (LBrace, statements, RBrace)
    parser.expect(TokenKind::LBrace)?;

    let mut statements = Vec::new();
    while parser.peek().kind() != TokenKind::RBrace {
        match parse_stmnt(parser) {
            Ok(stmnt) => statements.push(stmnt),
            Err(e) => {
                parser.push_error(e);
                parser.synchronize();
            }
        }
    }

    parser.expect(TokenKind::RBrace)?;

    Ok(ParsedDefn::make_fn(
        loc,
        header.0,
        header.1,
        return_type,
        statements,
    ))
}

/// Parse a function signature: <id>(<binding>, ...)
/// Returns the ID and a list of bindings if successful.
/// Returns an error otherwise.
fn parse_fn_sig<'a>(
    parser: &mut Parser<'a>,
) -> ParseResult<'a, (&'a str, Vec<SpannedBinding<'a>>)> {
    let id = parser.expect_id()?;
    let bindings = parse_binding_list(parser)?;
    Ok((id, bindings))
}
