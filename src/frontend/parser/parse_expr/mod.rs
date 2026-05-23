use crate::frontend::error::parse_error::{ParseError, ParseResult};
use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::{Binding, Expr, ExprKind, ParsedExpr};
use crate::frontend::parser::parse_type::{parse_binding, parse_binding_list};

#[cfg(test)]
mod tests;

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
enum Precedence {
    NotBinOp = -1,
    Pipe = 1, // pipe: |>
    Or = 2,   // logical or: ||
    And = 3,  // logical and: &&
    Eq = 4,   // equality: ==, !=
    Rel = 5,  // relational: <, <=, >, >=
    Add = 6,  // additive: +, -
    Mult = 7, // multiplicative: *, /
    Call = 8, // function call (postfix)
}

impl Precedence {
    fn of(token: &Token) -> Precedence {
        match token {
            Token::Times | Token::Divide => Precedence::Mult,
            Token::Plus | Token::Minus => Precedence::Add,
            Token::LessThan | Token::LessThanOrEq | Token::GreaterThan | Token::GreaterThanOrEq => Precedence::Rel,
            Token::Eq | Token::NotEq => Precedence::Eq,
            Token::And => Precedence::And,
            Token::Or => Precedence::Or,
            Token::Pipe => Precedence::Pipe,
            Token::LParen => Precedence::Call,
            _ => Precedence::NotBinOp,
        }
    }
}

pub(super) fn parse_expr<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    parse_expr_prec(parser, Precedence::NotBinOp)
}

fn parse_expr_prec<'a>(parser: &'a mut Parser<'a>, min_prec: Precedence) -> ParseResult<'a, ParsedExpr<'a>> {
    let mut left = parse_null_denotation(parser)?;

    loop {
        let prec = Precedence::of(&parser.peek().token);
        if prec <= min_prec {
            break;
        }

        // function call (postfix): left(arg, ...)
        if parser.peek().kind() == TokenKind::LParen {
            left = parse_call_expr(parser, left)?;
            continue;
        }

        // Binary operator
        let op = parser.advance().token;
        let right = parse_expr_prec(parser, prec)?;
        left = Expr::parsed(ExprKind::binop(op, left, right));
    }

    Ok(left)
}

// Null denotation: atoms and prefix operators.
fn parse_null_denotation<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    match parser.peek().kind() {
        TokenKind::Int | TokenKind::Float | TokenKind::Bool | TokenKind::Id => parse_atom_expr(parser),
        TokenKind::Minus | TokenKind::Not => parse_unary_expr(parser),
        TokenKind::LParen => parse_paren_expr(parser),
        TokenKind::Fn => parse_closure_expr(parser),
        TokenKind::If => parse_if_expr(parser),
        TokenKind::Match => parse_match_expr(parser),
        _ => ParseError::many(
            &[
                TokenKind::Int,
                TokenKind::Float,
                TokenKind::Bool,
                TokenKind::Id,
                TokenKind::Minus,
                TokenKind::Not,
                TokenKind::LParen,
                TokenKind::Fn,
                TokenKind::If,
                TokenKind::Match,
            ],
            parser.peek(),
        ),
    }
}

fn parse_atom_expr<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    match parser.peek().kind() {
        TokenKind::Int => Ok(Expr::parsed(ExprKind::Int(parser.expect_int()?))),
        TokenKind::Float => Ok(Expr::parsed(ExprKind::Float(parser.expect_float()?))),
        TokenKind::Bool => Ok(Expr::parsed(ExprKind::Bool(parser.expect_bool()?))),
        TokenKind::Id => Ok(Expr::parsed(ExprKind::Id(parser.expect_id()?))),
        _ => unreachable!(),
    }
}

fn parse_unary_expr<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    match parser.advance().token {
        Token::Minus => Ok(Expr::parsed(ExprKind::Neg(Box::new(parse_expr_prec(parser, Precedence::Mult)?)))),
        Token::Not => Ok(Expr::parsed(ExprKind::Bang(Box::new(parse_expr_prec(parser, Precedence::Mult)?)))),
        _ => unreachable!(),
    }
}

fn parse_paren_expr<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    parser.expect(TokenKind::LParen)?;
    let e = parse_expr(parser)?;
    parser.expect(TokenKind::RParen)?;
    Ok(e)
}

// fn ( <binding> ) -> <expr>
fn parse_closure_expr<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    parser.expect(TokenKind::Fn)?;
    parser.expect(TokenKind::LParen)?;
    let binding = parse_binding(parser)?;
    parser.expect(TokenKind::RParen)?;
    parser.expect(TokenKind::Arrow)?;
    let body = parse_expr(parser)?;
    Ok(Expr::parsed(ExprKind::Fn(binding, Box::new(body))))
}

// if <cond> then <then_branch> else <else_branch>
fn parse_if_expr<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    parser.expect(TokenKind::If)?;
    let cond = parse_expr(parser)?;
    parser.expect(TokenKind::Then)?;
    let then_branch = parse_expr(parser)?;
    parser.expect(TokenKind::Else)?;
    let else_branch = parse_expr(parser)?;
    Ok(Expr::parsed(ExprKind::If(Box::new(cond), Box::new(then_branch), Box::new(else_branch))))
}

// match <expr> { <id>(<binding>, ...) -> <expr>, ... }
fn parse_match_expr<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    parser.expect(TokenKind::Match)?;
    let scrutinee = parse_expr(parser)?;
    parser.expect(TokenKind::LBrace)?;
    let arms = parse_match_arms(parser)?;
    parser.expect(TokenKind::RBrace)?;
    Ok(Expr::parsed(ExprKind::Match(Box::new(scrutinee), arms)))
}

// <left>( <expr>, ... )
fn parse_call_expr<'a>(parser: &'a mut Parser<'a>, left: ParsedExpr<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    parser.expect(TokenKind::LParen)?;

    let mut args = Vec::new();

    if parser.peek().kind() != TokenKind::RParen {
        args.push(parse_expr(parser)?);
        while parser.peek().kind() == TokenKind::Comma {
            parser.advance();
            args.push(parse_expr(parser)?);
        }
    }

    parser.expect(TokenKind::RParen)?;
    Ok(Expr::parsed(ExprKind::Call(Box::new(left), args)))
}

// parse match arms: <id>(<binding>, ...) -> <expr>, ...
// arms are comma-separated and end at '}'.
fn parse_match_arms<'a>(parser: &'a mut Parser<'a>) -> ParseResult<'a, Vec<(&'a str, Vec<Binding<'a>>, ParsedExpr<'a>)>> {
    let mut arms = Vec::new();

    while parser.peek().kind() != TokenKind::RBrace {
        let constructor = parser.expect_id()?;
        let bindings = parse_binding_list(parser)?;
        parser.expect(TokenKind::Arrow)?;
        let body = parse_expr(parser)?;
        arms.push((constructor, bindings, body));

        if parser.peek().kind() == TokenKind::Comma {
            parser.advance();
        } else {
            break;
        }
    }

    Ok(arms)
}
