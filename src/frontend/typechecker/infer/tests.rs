use std::collections::HashMap;

use crate::frontend::{
    lexer,
    parser::{self, Parser, ast::ExprKind},
    typechecker::{
        infer::infer,
        types::{Env, TypeValue, TypedExpr},
    },
};

fn typeck(input: &str) -> TypedExpr<'_> {
    let mut parser = Parser::new(lexer::lex(input));
    let parsed_expr = parser::parse_expr::parse_expr(&mut parser).unwrap();
    let env = Env::Global(HashMap::new());
    infer(parsed_expr, &env).unwrap()
}

#[test]
fn test_int() {
    let expr = typeck("42");
    assert_eq!(expr, TypedExpr::new(0, TypeValue::Int, ExprKind::Int(42)));
}

#[test]
fn test_float() {
    let expr = typeck("3.14");
    assert_eq!(
        expr,
        TypedExpr::new(0, TypeValue::Float, ExprKind::Float(3.14))
    );
}

#[test]
fn test_bool() {
    let expr = typeck("true");
    assert_eq!(
        expr,
        TypedExpr::new(0, TypeValue::Bool, ExprKind::Bool(true))
    );
}

#[test]
fn test_add() {
    let expr = typeck("1 + 2");
    assert_eq!(
        expr,
        TypedExpr::new(
            0,
            TypeValue::Int,
            ExprKind::Plus(
                Box::new(TypedExpr::new(0, TypeValue::Int, ExprKind::Int(1))),
                Box::new(TypedExpr::new(4, TypeValue::Int, ExprKind::Int(2)))
            )
        )
    );
}

#[test]
fn test_nested() {
    let expr = typeck("1 + 2 * 3");
    assert_eq!(
        expr,
        TypedExpr::new(
            0,
            TypeValue::Int,
            ExprKind::Plus(
                Box::new(TypedExpr::new(0, TypeValue::Int, ExprKind::Int(1))),
                Box::new(TypedExpr::new(
                    4,
                    TypeValue::Int,
                    ExprKind::Mult(
                        Box::new(TypedExpr::new(4, TypeValue::Int, ExprKind::Int(2))),
                        Box::new(TypedExpr::new(8, TypeValue::Int, ExprKind::Int(3)))
                    )
                ))
            )
        )
    );
}

#[test]
fn test_parens() {
    let expr = typeck("(1 + 2) * 3");
    assert_eq!(
        expr,
        TypedExpr::new(
            0,
            TypeValue::Int,
            ExprKind::Mult(
                Box::new(TypedExpr::new(
                    0,
                    TypeValue::Int,
                    ExprKind::Plus(
                        Box::new(TypedExpr::new(1, TypeValue::Int, ExprKind::Int(1))),
                        Box::new(TypedExpr::new(5, TypeValue::Int, ExprKind::Int(2)))
                    )
                )),
                Box::new(TypedExpr::new(10, TypeValue::Int, ExprKind::Int(3)))
            )
        )
    );
}

#[test]
fn test_relational() {
    let expr = typeck("1 < 2");
    assert_eq!(
        expr,
        TypedExpr::new(
            0,
            TypeValue::Bool,
            ExprKind::Less(
                Box::new(TypedExpr::new(0, TypeValue::Int, ExprKind::Int(1))),
                Box::new(TypedExpr::new(4, TypeValue::Int, ExprKind::Int(2)))
            )
        )
    );
}

#[test]
fn test_logical() {
    let expr = typeck("true && false");
    assert_eq!(
        expr,
        TypedExpr::new(
            0,
            TypeValue::Bool,
            ExprKind::And(
                Box::new(TypedExpr::new(0, TypeValue::Bool, ExprKind::Bool(true))),
                Box::new(TypedExpr::new(8, TypeValue::Bool, ExprKind::Bool(false)))
            )
        )
    );
}

#[test]
fn test_if() {
    let expr = typeck("if true then 1 else 2");
    assert_eq!(
        expr,
        TypedExpr::new(
            0,
            TypeValue::Int,
            ExprKind::If(
                Box::new(TypedExpr::new(3, TypeValue::Bool, ExprKind::Bool(true))),
                Box::new(TypedExpr::new(13, TypeValue::Int, ExprKind::Int(1))),
                Box::new(TypedExpr::new(20, TypeValue::Int, ExprKind::Int(2)))
            )
        )
    );
}
