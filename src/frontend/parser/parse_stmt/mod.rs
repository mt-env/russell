use crate::frontend::lexer::token::TokenKind;
use crate::frontend::parser::ast::ParsedStmt;
use crate::frontend::parser::parse_expr::parse_expr;
use crate::frontend::parser::parse_type::parse_type;
use crate::frontend::error::parse_error::{ParseError, ParseResult};
use crate::frontend::parser::Parser;

#[cfg(test)]
mod tests;

pub(super) fn parse_stmnt(parser: &mut Parser) -> ParseResult<ParsedStmt> {
    match parser.peek().kind() {
        TokenKind::Let => parse_let(parser),
        TokenKind::Read => parse_read(parser),
        TokenKind::Echo => parse_echo(parser),
        TokenKind::Return => parse_return(parser),
        _ => ParseError::many(
            &[TokenKind::Let, TokenKind::Read, TokenKind::Echo, TokenKind::Return],
            parser.peek(),
        ),
    }
}

fn parse_let(parser: &mut Parser) -> ParseResult<ParsedStmt> {
    parser.expect(TokenKind::Let)?;
    let id = parser.expect_id()?;
    parser.expect(TokenKind::Assign)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::Let(id, expr))
}

fn parse_read(parser: &mut Parser) -> ParseResult<ParsedStmt> {
    parser.expect(TokenKind::Read)?;
    let read_type = parse_type(parser)?;
    let id = parser.expect_id()?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::Read(read_type, id))
}

fn parse_echo(parser: &mut Parser) -> ParseResult<ParsedStmt> {
    parser.expect(TokenKind::Echo)?;
    let echo_type = parse_type(parser)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::Echo(echo_type, expr))
}

fn parse_return(parser: &mut Parser) -> ParseResult<ParsedStmt> {
    parser.expect(TokenKind::Return)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::Return(expr))
}
