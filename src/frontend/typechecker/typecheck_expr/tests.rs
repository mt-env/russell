use std::collections::HashMap;

use crate::frontend::{
    lexer,
    parser::{self, Parser, ast::ExprKind},
    typechecker::{
        typecheck_expr::typecheck_expr,
        types::{Env, TypeValue, TypedExpr},
    },
};

fn typeck(input: &str) -> TypedExpr<'_> {
    let mut parser = Parser::new(lexer::lex(input));
    let parsed_expr = parser::parse_expr::parse_expr(&mut parser).unwrap();
    let env = Env::Global(HashMap::new());
    typecheck_expr(parsed_expr, &env).unwrap()
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
