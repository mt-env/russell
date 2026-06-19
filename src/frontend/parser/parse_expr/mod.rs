use crate::frontend::error::parse_error::{ParseError, ParseResult};
use crate::frontend::lexer::token::{SpannedToken, TokenKind};
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::{ExprKind, ParsedBinding, ParsedExpr};
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
    fn of(token: &SpannedToken) -> Precedence {
        match token.kind() {
            TokenKind::Times | TokenKind::Divide => Precedence::Mult,
            TokenKind::Plus | TokenKind::Minus => Precedence::Add,
            TokenKind::LessThan
            | TokenKind::LessThanOrEq
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanOrEq => Precedence::Rel,
            TokenKind::Eq | TokenKind::NotEq => Precedence::Eq,
            TokenKind::And => Precedence::And,
            TokenKind::Or => Precedence::Or,
            TokenKind::Pipe => Precedence::Pipe,
            TokenKind::LParen => Precedence::Call,
            _ => Precedence::NotBinOp,
        }
    }
}

pub fn parse_expr<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    parse_expr_prec(parser, Precedence::NotBinOp)
}

fn parse_expr_prec<'a>(
    parser: &mut Parser<'a>,
    min_prec: Precedence,
) -> ParseResult<'a, ParsedExpr<'a>> {
    let mut left = parse_null_denotation(parser)?;

    loop {
        let prec = Precedence::of(parser.peek());
        if prec <= min_prec {
            break;
        }

        // function call (postfix): left(arg, ...)
        if parser.peek().kind() == TokenKind::LParen {
            left = parse_call_expr(parser, left)?;
            continue;
        }

        // Binary operator
        let op = parser.advance().kind();
        let right = parse_expr_prec(parser, prec)?;
        left = ParsedExpr::new(left.offset, ExprKind::binop(op, left, right));
    }

    Ok(left)
}

// Null denotation: atoms and prefix operators.
fn parse_null_denotation<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    match parser.peek().kind() {
        TokenKind::Int | TokenKind::Float | TokenKind::Bool | TokenKind::Id => {
            parse_atom_expr(parser)
        }
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

fn parse_atom_expr<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    let loc = parser.peek().offset;
    match parser.peek().kind() {
        TokenKind::Int => Ok(ParsedExpr::new(loc, ExprKind::Int(parser.expect_int()?))),
        TokenKind::Float => Ok(ParsedExpr::new(
            loc,
            ExprKind::Float(parser.expect_float()?),
        )),
        TokenKind::Bool => Ok(ParsedExpr::new(loc, ExprKind::Bool(parser.expect_bool()?))),
        TokenKind::Id => Ok(ParsedExpr::new(loc, ExprKind::Id(parser.expect_id()?))),
        _ => unreachable!(),
    }
}

fn parse_unary_expr<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    let loc = parser.peek().offset;
    match parser.advance().kind() {
        TokenKind::Minus => Ok(ParsedExpr::new(
            loc,
            ExprKind::Neg(Box::new(parse_expr_prec(parser, Precedence::Mult)?)),
        )),
        TokenKind::Not => Ok(ParsedExpr::new(
            loc,
            ExprKind::Bang(Box::new(parse_expr_prec(parser, Precedence::Mult)?)),
        )),
        _ => unreachable!(),
    }
}

fn parse_paren_expr<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::LParen)?;
    let e = parse_expr(parser)?;
    parser.expect(TokenKind::RParen)?;
    Ok(ParsedExpr { offset: loc, ..e })
}

// fn ( <binding> ) -> <expr>
fn parse_closure_expr<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Fn)?;
    parser.expect(TokenKind::LParen)?;
    let binding = parse_binding(parser)?;
    parser.expect(TokenKind::RParen)?;
    parser.expect(TokenKind::Arrow)?;
    let body = parse_expr(parser)?;
    Ok(ParsedExpr::new(loc, ExprKind::Fn(binding, Box::new(body))))
}

// if <cond> then <then_branch> else <else_branch>
fn parse_if_expr<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::If)?;
    let cond = parse_expr(parser)?;
    parser.expect(TokenKind::Then)?;
    let then_branch = parse_expr(parser)?;
    parser.expect(TokenKind::Else)?;
    let else_branch = parse_expr(parser)?;
    Ok(ParsedExpr::new(
        loc,
        ExprKind::If(Box::new(cond), Box::new(then_branch), Box::new(else_branch)),
    ))
}

// match <expr> { <id>(<binding>, ...) -> <expr>, ... }
fn parse_match_expr<'a>(parser: &mut Parser<'a>) -> ParseResult<'a, ParsedExpr<'a>> {
    let loc = parser.peek().offset;
    parser.expect(TokenKind::Match)?;
    let scrutinee = parse_expr(parser)?;
    parser.expect(TokenKind::LBrace)?;
    let arms = parse_match_arms(parser)?;
    parser.expect(TokenKind::RBrace)?;
    Ok(ParsedExpr::new(
        loc,
        ExprKind::Match(Box::new(scrutinee), arms),
    ))
}

// <left>( <expr>, ... )
fn parse_call_expr<'a>(
    parser: &mut Parser<'a>,
    left: ParsedExpr<'a>,
) -> ParseResult<'a, ParsedExpr<'a>> {
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
    Ok(ParsedExpr::new(
        left.offset,
        ExprKind::Call(Box::new(left), args),
    ))
}

// parse match arms: <id>(<binding>, ...) -> <expr>, ...
// arms are comma-separated and end at '}'.
fn parse_match_arms<'a>(
    parser: &mut Parser<'a>,
) -> ParseResult<'a, Vec<(&'a str, Vec<ParsedBinding<'a>>, ParsedExpr<'a>)>> {
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
