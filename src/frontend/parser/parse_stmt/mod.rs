use crate::frontend::error::parse_error::{ParseError, ParseResult};
use crate::frontend::lexer::token::TokenKind;
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::ParsedStmt;
use crate::frontend::parser::parse_expr::parse_expr;
use crate::frontend::parser::parse_type::parse_type;

#[cfg(test)]
mod tests;

pub(super) fn parse_stmnt<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedStmt<'a>> {
    match parser.peek().kind() {
        TokenKind::Let => parse_let(parser),
        TokenKind::Read => parse_read(parser),
        TokenKind::Echo => parse_echo(parser),
        TokenKind::Return => parse_return(parser),
        _ => ParseError::many(
            &[
                TokenKind::Let,
                TokenKind::Read,
                TokenKind::Echo,
                TokenKind::Return,
            ],
            parser.peek(),
        ),
    }
}

fn parse_let<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedStmt<'a>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Let)?;
    let id = parser.expect_id()?;
    parser.expect(TokenKind::Assign)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::make_let(loc, id, expr))
}

fn parse_read<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedStmt<'a>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Read)?;
    let read_type = parse_type(parser)?;
    let id = parser.expect_id()?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::make_read(loc, read_type, id))
}

fn parse_echo<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedStmt<'a>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Echo)?;
    let echo_type = parse_type(parser)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::make_echo(loc, echo_type, expr))
}

fn parse_return<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedStmt<'a>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Return)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::make_return(loc, expr))
}
