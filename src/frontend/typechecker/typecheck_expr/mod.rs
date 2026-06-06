use crate::frontend::{
    parser::ast::{Expr, ExprKind, ParsedExpr},
    typechecker::types::{Env, TypeResult, TypeValue, TypedExpr},
};

pub(super) fn typecheck_expr(expr: ParsedExpr) -> TypeResult<TypedExpr> {
    match expr.kind {
        ExprKind::Int(n) => Ok(TypedExpr::new(TypeValue::Int, ExprKind::Int(n))),
        ExprKind::Float(n) => Ok(TypedExpr::new(TypeValue::Float, ExprKind::Float(n))),
        ExprKind::Bool(val) => Ok(TypedExpr::new(TypeValue::Bool, ExprKind::Bool(val))),
        ExprKind::Id(_) => todo!(),
        ExprKind::Fn(binding, expr) => todo!(),
        ExprKind::Neg(expr) => todo!(),
        ExprKind::Bang(expr) => todo!(),
        ExprKind::Call(expr, exprs) => todo!(),
        ExprKind::Plus(expr, expr1) => todo!(),
        ExprKind::Minus(expr, expr1) => todo!(),
        ExprKind::Mult(expr, expr1) => todo!(),
        ExprKind::Div(expr, expr1) => todo!(),
        ExprKind::Pipe(expr, expr1) => todo!(),
        ExprKind::Less(expr, expr1) => todo!(),
        ExprKind::LessEq(expr, expr1) => todo!(),
        ExprKind::Greater(expr, expr1) => todo!(),
        ExprKind::GreaterEq(expr, expr1) => todo!(),
        ExprKind::Eq(expr, expr1) => todo!(),
        ExprKind::NotEq(expr, expr1) => todo!(),
        ExprKind::Or(expr, expr1) => todo!(),
        ExprKind::And(expr, expr1) => todo!(),
        ExprKind::If(expr, expr1, expr2) => todo!(),
        ExprKind::Match(expr, items) => todo!(),
    }
}

fn typecheck_id(id: String, env: &Env) -> TypeResult<TypeValue> {
    todo!()
}

fn typecheck_fn(binding: String, expr: ParsedExpr, env: &Env) -> TypeResult<TypeValue> {
    todo!()
}
