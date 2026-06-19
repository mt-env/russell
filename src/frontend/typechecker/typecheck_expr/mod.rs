use crate::frontend::{
    parser::ast::{ExprKind, ParsedExpr},
    typechecker::types::{Env, TypeError, TypeResult, TypeValue, TypedExpr},
};

#[cfg(test)]
mod tests;

pub(super) fn typecheck_expr<'a>(expr: ParsedExpr<'a>, env: &Env) -> TypeResult<TypedExpr<'a>> {
    match expr.node.kind {
        ExprKind::Int(n) => Ok(TypedExpr::new(
            expr.offset,
            TypeValue::Int,
            ExprKind::Int(n),
        )),
        ExprKind::Float(n) => Ok(TypedExpr::new(
            expr.offset,
            TypeValue::Float,
            ExprKind::Float(n),
        )),
        ExprKind::Bool(val) => Ok(TypedExpr::new(
            expr.offset,
            TypeValue::Bool,
            ExprKind::Bool(val),
        )),
        ExprKind::Id(s) => typecheck_id(expr.offset, s, env),
        ExprKind::Fn(binding, expr) => todo!(),
        ExprKind::Neg(expr) => typecheck_neg(*expr, env),
        ExprKind::Bang(expr) => typecheck_bang(*expr, env),
        ExprKind::Call(left, right) => todo!(),
        ExprKind::Plus(left, right) => todo!(),
        ExprKind::Minus(left, right) => todo!(),
        ExprKind::Mult(left, right) => todo!(),
        ExprKind::Div(left, right) => todo!(),
        ExprKind::Pipe(left, right) => todo!(),
        ExprKind::Less(left, right) => todo!(),
        ExprKind::LessEq(left, right) => todo!(),
        ExprKind::Greater(left, right) => todo!(),
        ExprKind::GreaterEq(left, right) => todo!(),
        ExprKind::Eq(left, right) => todo!(),
        ExprKind::NotEq(left, right) => todo!(),
        ExprKind::Or(left, right) => todo!(),
        ExprKind::And(left, right) => todo!(),
        ExprKind::If(cond, thenb, elseb) => todo!(),
        ExprKind::Match(expr, arms) => todo!(),
    }
}

fn typecheck_id<'a>(offset: usize, id: &'a str, env: &Env) -> TypeResult<TypedExpr<'a>> {
    match env.lookup(&id) {
        Some(ty) => Ok(TypedExpr::new(offset, ty, ExprKind::Id(id))),
        None => Err(TypeError {
            expected: TypeValue::Var(Box::new(None)),
            actual: TypeValue::Var(Box::new(None)),
            offset,
        }),
    }
}

fn typecheck_fn(binding: String, expr: ParsedExpr, env: &Env) -> TypeResult<TypeValue> {
    todo!()
}

fn typecheck_neg<'a>(expr: ParsedExpr<'a>, env: &Env) -> TypeResult<TypedExpr<'a>> {
    let checked_expr = typecheck_expr(expr, env)?;
    match checked_expr.ann() {
        TypeValue::Int => Ok(TypedExpr::new(
            checked_expr.offset,
            TypeValue::Int,
            ExprKind::Neg(Box::new(checked_expr)),
        )),
        TypeValue::Float => Ok(TypedExpr::new(
            checked_expr.offset,
            TypeValue::Float,
            ExprKind::Neg(Box::new(checked_expr)),
        )),
        _ => Err(TypeError {
            expected: TypeValue::Int,
            actual: checked_expr.ann(),
            offset: checked_expr.offset,
        }),
    }
}

fn typecheck_bang<'a>(expr: ParsedExpr<'a>, env: &Env) -> TypeResult<TypedExpr<'a>> {
    let checked_expr = typecheck_expr(expr, env)?;
    match checked_expr.ann() {
        TypeValue::Bool => Ok(TypedExpr::new(
            checked_expr.offset,
            TypeValue::Bool,
            ExprKind::Bang(Box::new(checked_expr)),
        )),
        _ => Err(TypeError {
            expected: TypeValue::Bool,
            actual: checked_expr.ann(),
            offset: checked_expr.offset,
        }),
    }
}
