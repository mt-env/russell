use crate::frontend::lexer::lex;
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::{ExprKind, ParsedExpr, ParsedStmt, Type};

fn parser_from(input: &str) -> Parser {
    Parser::new(lex(input))
}

fn parse(input: &str) -> ParsedStmt {
    let mut p = parser_from(input);
    super::parse_stmnt(&mut p).unwrap()
}

// ─── let ────────────────────────────────────────────────────────────

#[test]
fn let_int_literal() {
    assert_eq!(
        parse("let x = 42;"),
        ParsedStmt::make_let(0, "x".into(), ParsedExpr::new(8, ExprKind::Int(42)))
    );
}

#[test]
fn let_bool_literal() {
    assert_eq!(
        parse("let flag = true;"),
        ParsedStmt::make_let(0, "flag".into(), ParsedExpr::new(11, ExprKind::Bool(true)))
    );
}

#[test]
fn let_with_binary_expr() {
    assert_eq!(
        parse("let x = 1 + 2;"),
        ParsedStmt::make_let(
            0,
            "x".into(),
            ParsedExpr::new(
                8,
                ExprKind::Plus(
                    Box::new(ParsedExpr::new(8, ExprKind::Int(1))),
                    Box::new(ParsedExpr::new(12, ExprKind::Int(2)))
                )
            )
        )
    );
}

#[test]
fn let_with_identifier() {
    assert_eq!(
        parse("let y = x;"),
        ParsedStmt::make_let(0, "y".into(), ParsedExpr::new(8, ExprKind::Id("x".into())))
    );
}

#[test]
fn let_error_missing_semicolon() {
    let mut p = parser_from("let x = 42");
    assert!(super::parse_stmnt(&mut p).is_err());
}

#[test]
fn let_error_missing_assign() {
    let mut p = parser_from("let x 42;");
    assert!(super::parse_stmnt(&mut p).is_err());
}

// ─── read ───────────────────────────────────────────────────────────

#[test]
fn read_int() {
    assert_eq!(parse("read Int x;"), ParsedStmt::make_read(0, Type::Int, "x".into()));
}

#[test]
fn read_float() {
    assert_eq!(
        parse("read Float y;"),
        ParsedStmt::make_read(0, Type::Float, "y".into())
    );
}

#[test]
fn read_bool() {
    assert_eq!(parse("read Bool z;"), ParsedStmt::make_read(0, Type::Bool, "z".into()));
}

#[test]
fn read_error_missing_semicolon() {
    let mut p = parser_from("read Int x");
    assert!(super::parse_stmnt(&mut p).is_err());
}

#[test]
fn read_error_missing_type() {
    let mut p = parser_from("read x;");
    assert!(super::parse_stmnt(&mut p).is_err());
}

// ─── echo ───────────────────────────────────────────────────────────

#[test]
fn echo_int_literal() {
    assert_eq!(
        parse("echo Int 42;"),
        ParsedStmt::make_echo(0, Type::Int, ParsedExpr::new(9, ExprKind::Int(42)))
    );
}

#[test]
fn echo_float_literal() {
    assert_eq!(
        parse("echo Float 3.14;"),
        ParsedStmt::make_echo(0, Type::Float, ParsedExpr::new(11, ExprKind::Float(3.14)))
    );
}

#[test]
fn echo_bool_literal() {
    assert_eq!(
        parse("echo Bool true;"),
        ParsedStmt::make_echo(0, Type::Bool, ParsedExpr::new(10, ExprKind::Bool(true)))
    );
}

#[test]
fn echo_with_expression() {
    assert_eq!(
        parse("echo Int x + 1;"),
        ParsedStmt::make_echo(
            0,
            Type::Int,
            ParsedExpr::new(
                9,
                ExprKind::Plus(
                    Box::new(ParsedExpr::new(9, ExprKind::Id("x".into()))),
                    Box::new(ParsedExpr::new(13, ExprKind::Int(1)))
                )
            )
        )
    );
}

#[test]
fn echo_error_missing_semicolon() {
    let mut p = parser_from("echo Int 42");
    assert!(super::parse_stmnt(&mut p).is_err());
}

// ─── return ─────────────────────────────────────────────────────────

#[test]
fn return_int_literal() {
    assert_eq!(
        parse("return 42;"),
        ParsedStmt::make_return(0, ParsedExpr::new(7, ExprKind::Int(42)))
    );
}

#[test]
fn return_identifier() {
    assert_eq!(
        parse("return x;"),
        ParsedStmt::make_return(0, ParsedExpr::new(7, ExprKind::Id("x".into())))
    );
}

#[test]
fn return_with_expression() {
    assert_eq!(
        parse("return a + b;"),
        ParsedStmt::make_return(
            0,
            ParsedExpr::new(
                7,
                ExprKind::Plus(
                    Box::new(ParsedExpr::new(7, ExprKind::Id("a".into()))),
                    Box::new(ParsedExpr::new(11, ExprKind::Id("b".into())))
                )
            )
        )
    );
}

#[test]
fn return_error_missing_semicolon() {
    let mut p = parser_from("return 42");
    assert!(super::parse_stmnt(&mut p).is_err());
}

// ─── dispatch errors ────────────────────────────────────────────────

#[test]
fn error_on_int_literal() {
    let mut p = parser_from("42;");
    assert!(super::parse_stmnt(&mut p).is_err());
}

#[test]
fn error_on_identifier() {
    let mut p = parser_from("foo;");
    assert!(super::parse_stmnt(&mut p).is_err());
}

#[test]
fn error_on_operator() {
    let mut p = parser_from("+;");
    assert!(super::parse_stmnt(&mut p).is_err());
}
