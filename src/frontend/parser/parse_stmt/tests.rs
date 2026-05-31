use crate::frontend::lexer::lex;
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::{Expr, ExprKind, ParsedStmt, Stmt, Type};

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
        Stmt::Let("x".into(), Expr::parsed(ExprKind::Int(42)))
    );
}

#[test]
fn let_bool_literal() {
    assert_eq!(
        parse("let flag = true;"),
        Stmt::Let("flag".into(), Expr::parsed(ExprKind::Bool(true)))
    );
}

#[test]
fn let_with_binary_expr() {
    assert_eq!(
        parse("let x = 1 + 2;"),
        Stmt::Let(
            "x".into(),
            Expr::parsed(ExprKind::Plus(
                Box::new(Expr::parsed(ExprKind::Int(1))),
                Box::new(Expr::parsed(ExprKind::Int(2)))
            ))
        )
    );
}

#[test]
fn let_with_identifier() {
    assert_eq!(
        parse("let y = x;"),
        Stmt::Let("y".into(), Expr::parsed(ExprKind::Id("x".into())))
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
    assert_eq!(parse("read Int x;"), Stmt::Read(Type::Int, "x".into()));
}

#[test]
fn read_float() {
    assert_eq!(parse("read Float y;"), Stmt::Read(Type::Float, "y".into()));
}

#[test]
fn read_bool() {
    assert_eq!(parse("read Bool z;"), Stmt::Read(Type::Bool, "z".into()));
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
        Stmt::Echo(Type::Int, Expr::parsed(ExprKind::Int(42)))
    );
}

#[test]
fn echo_float_literal() {
    assert_eq!(
        parse("echo Float 3.14;"),
        Stmt::Echo(Type::Float, Expr::parsed(ExprKind::Float(3.14)))
    );
}

#[test]
fn echo_bool_literal() {
    assert_eq!(
        parse("echo Bool true;"),
        Stmt::Echo(Type::Bool, Expr::parsed(ExprKind::Bool(true)))
    );
}

#[test]
fn echo_with_expression() {
    assert_eq!(
        parse("echo Int x + 1;"),
        Stmt::Echo(
            Type::Int,
            Expr::parsed(ExprKind::Plus(
                Box::new(Expr::parsed(ExprKind::Id("x".into()))),
                Box::new(Expr::parsed(ExprKind::Int(1)))
            ))
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
    assert_eq!(parse("return 42;"), Stmt::Return(Expr::parsed(ExprKind::Int(42))));
}

#[test]
fn return_identifier() {
    assert_eq!(parse("return x;"), Stmt::Return(Expr::parsed(ExprKind::Id("x".into()))));
}

#[test]
fn return_with_expression() {
    assert_eq!(
        parse("return a + b;"),
        Stmt::Return(Expr::parsed(ExprKind::Plus(
            Box::new(Expr::parsed(ExprKind::Id("a".into()))),
            Box::new(Expr::parsed(ExprKind::Id("b".into())))
        )))
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
