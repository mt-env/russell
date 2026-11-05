use crate::frontend::{
    error::parse_error::{ParseError, ParseResult},
    lexer::token::TokenKind,
    parser::{Parser, ast::ParsedStmt, parse_expr::parse_expr, parse_type::parse_type},
};

#[cfg(test)]
mod tests;

pub(crate) fn parse_stmt<'src>(parser: &mut Parser<'src>) -> ParseResult<'src, ParsedStmt<'src>> {
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

fn parse_let<'src>(parser: &mut Parser<'src>) -> ParseResult<'src, ParsedStmt<'src>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Let)?;
    let id = parser.expect_id()?;
    parser.expect(TokenKind::Assign)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::make_let(loc, id, expr))
}

fn parse_read<'src>(parser: &mut Parser<'src>) -> ParseResult<'src, ParsedStmt<'src>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Read)?;
    let read_type = parse_type(parser)?;
    let id = parser.expect_id()?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::make_read(loc, read_type, id))
}

fn parse_echo<'src>(parser: &mut Parser<'src>) -> ParseResult<'src, ParsedStmt<'src>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Echo)?;
    let echo_type = parse_type(parser)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::make_echo(loc, echo_type, expr))
}

fn parse_return<'src>(parser: &mut Parser<'src>) -> ParseResult<'src, ParsedStmt<'src>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Return)?;
    let expr = parse_expr(parser)?;
    parser.expect(TokenKind::Semicolon)?;
    Ok(ParsedStmt::make_return(loc, expr))
}
