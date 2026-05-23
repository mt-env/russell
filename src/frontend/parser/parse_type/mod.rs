use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::{Binding, Type};
use crate::frontend::error::parse_error::ParseResult;
use crate::frontend::parser::Parser;

#[cfg(test)]
mod tests;

pub(super) fn parse_type<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, Type<'a>> {
    // parse an atomic type
    let l_type = match parser.expect_many(&[
        TokenKind::IntType,
        TokenKind::FloatType,
        TokenKind::BoolType,
        TokenKind::TypeId,
    ])? {
        Token::IntType => Type::Int,
        Token::FloatType => Type::Float,
        Token::BoolType => Type::Bool,
        Token::TypeId(id) => Type::TypeId(id.clone()),
        _ => unreachable!(),
    };

    // if we see an arrow, parse the right-hand side of the function type
    if parser.peek().kind() == TokenKind::Arrow {
        parser.advance();
        let r_type = parse_type(parser)?;
        return Ok(Type::Fn(Box::new(l_type), Box::new(r_type)));
    }

    // otherwise, there's no right-hand side
    Ok(l_type)
}

pub(super) fn parse_binding<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, Binding<'a>> {
    let id = parser.expect_id()?;
    parser.expect(TokenKind::Colon)?;
    let id_type = parse_type(parser)?;
    Ok(Binding::new(id, id_type))
}

pub(super) fn parse_binding_list<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, Vec<Binding<'a>>> {
    parser.expect(TokenKind::LParen)?;

    let mut bindings = Vec::new();

    if parser.peek().kind() != TokenKind::RParen {
        bindings.push(parse_binding(parser)?);
        while parser.peek().kind() == TokenKind::Comma {
            parser.advance();
            bindings.push(parse_binding(parser)?);
        }
    }

    parser.expect(TokenKind::RParen)?;
    Ok(bindings)
}
