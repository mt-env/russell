use crate::frontend::{parser::ast::Expr, typechecker::types::TypedExpr};


pub(super) fn typecheck_expr(expr: Expr) -> TypedExpr {
    match expr {
        Expr::Int(_) => todo!(),
        Expr::Float(_) => todo!(),
        Expr::Bool(_) => todo!(),
        Expr::Id(_) => todo!(),
        Expr::Fn(binding, expr) => todo!(),
        Expr::Neg(expr) => todo!(),
        Expr::Bang(expr) => todo!(),
        Expr::Call(expr, exprs) => todo!(),
        Expr::Plus(left, right) => todo!(),
        Expr::Minus(left, right) => todo!(),
        Expr::Mult(left, right) => todo!(),
        Expr::Div(left, right) => todo!(),
        Expr::Pipe(left, right) => todo!(),
        Expr::Less(left, right) => todo!(),
        Expr::LessEq(left, right) => todo!(),
        Expr::Greater(left, right) => todo!(),
        Expr::GreaterEq(left, right) => todo!(),
        Expr::Eq(left, right) => todo!(),
        Expr::NotEq(left, right) => todo!(),
        Expr::Or(left, right) => todo!(),
        Expr::And(left, right) => todo!(),
        Expr::If(expr, expr1, expr2) => todo!(),
        Expr::Match(expr, items) => todo!(),
    }
}
