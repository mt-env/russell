use crate::frontend::{
    parser::ast::{ExprKind, ParsedExpr, SpannedBinding, SpannedExpr},
    typechecker::{
        check::{check, unify},
        types::{Env, TypeError, TypeResult, TypeValue, TypedExpr},
    },
};

#[cfg(test)]
mod tests;

pub(super) fn infer<'a>(expr: ParsedExpr<'a>, env: &Env) -> TypeResult<TypedExpr<'a>> {
    let loc = expr.offset;
    match expr.node.kind {
        ExprKind::Int(n) => Ok(TypedExpr::new(loc, TypeValue::Int, ExprKind::Int(n))),
        ExprKind::Float(n) => Ok(TypedExpr::new(loc, TypeValue::Float, ExprKind::Float(n))),
        ExprKind::Bool(val) => Ok(TypedExpr::new(loc, TypeValue::Bool, ExprKind::Bool(val))),
        ExprKind::Id(s) => infer_id(loc, s, env),
        ExprKind::Fn(binding, expr) => todo!(),
        ExprKind::Neg(expr) => infer_neg(loc, *expr, env),
        ExprKind::Bang(expr) => infer_bang(loc, *expr, env),
        ExprKind::Call(left, right) => todo!(),
        ExprKind::Plus(left, right) => infer_arith_binop(loc, *left, *right, ExprKind::Plus, env),
        ExprKind::Minus(left, right) => infer_arith_binop(loc, *left, *right, ExprKind::Minus, env),
        ExprKind::Mult(left, right) => infer_arith_binop(loc, *left, *right, ExprKind::Mult, env),
        ExprKind::Div(left, right) => infer_arith_binop(loc, *left, *right, ExprKind::Div, env),
        ExprKind::Pipe(left, right) => todo!(),
        ExprKind::Less(left, right) => infer_cmp_binop(loc, *left, *right, ExprKind::Less, env),
        ExprKind::LessEq(left, right) => infer_cmp_binop(loc, *left, *right, ExprKind::LessEq, env),
        ExprKind::Greater(left, right) => {
            infer_cmp_binop(loc, *left, *right, ExprKind::Greater, env)
        }
        ExprKind::GreaterEq(left, right) => {
            infer_cmp_binop(loc, *left, *right, ExprKind::GreaterEq, env)
        }
        ExprKind::Eq(left, right) => infer_cmp_binop(loc, *left, *right, ExprKind::Eq, env),
        ExprKind::NotEq(left, right) => infer_cmp_binop(loc, *left, *right, ExprKind::NotEq, env),
        ExprKind::Or(left, right) => infer_bool_binop(loc, *left, *right, ExprKind::Or, env),
        ExprKind::And(left, right) => infer_bool_binop(loc, *left, *right, ExprKind::And, env),
        ExprKind::If(cond, thenb, elseb) => infer_if(loc, *cond, *thenb, *elseb, env),
        ExprKind::Match(expr, arms) => infer_match(loc, *expr, arms, env),
    }
}

fn infer_id<'a>(offset: usize, id: &'a str, env: &Env) -> TypeResult<TypedExpr<'a>> {
    match env.lookup(&id) {
        Some(ty) => Ok(TypedExpr::new(offset, ty, ExprKind::Id(id))),
        None => Err(TypeError {
            expected: TypeValue::Var(Box::new(None)),
            actual: TypeValue::Var(Box::new(None)),
            offset,
        }),
    }
}

fn infer_fn(binding: String, expr: ParsedExpr, env: &Env) -> TypeResult<TypeValue> {
    todo!()
}

fn infer_neg<'a>(offset: usize, expr: ParsedExpr<'a>, env: &Env) -> TypeResult<TypedExpr<'a>> {
    let checked_expr = infer(expr, env)?;
    match checked_expr.ty() {
        TypeValue::Int => Ok(TypedExpr::new(
            offset,
            TypeValue::Int,
            ExprKind::Neg(Box::new(checked_expr)),
        )),
        TypeValue::Float => Ok(TypedExpr::new(
            offset,
            TypeValue::Float,
            ExprKind::Neg(Box::new(checked_expr)),
        )),
        _ => Err(TypeError {
            expected: TypeValue::Int,
            actual: checked_expr.ty(),
            offset,
        }),
    }
}

fn infer_bang<'a>(offset: usize, expr: ParsedExpr<'a>, env: &Env) -> TypeResult<TypedExpr<'a>> {
    let checked_expr = check(expr, TypeValue::Bool, env)?;
    Ok(TypedExpr::new(
        offset,
        TypeValue::Bool,
        ExprKind::Bang(Box::new(checked_expr)),
    ))
}

fn infer_call() {
    todo!()
}

fn infer_arith_binop<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    make: impl FnOnce(Box<TypedExpr<'a>>, Box<TypedExpr<'a>>) -> ExprKind<'a, TypeValue>,
    env: &Env,
) -> TypeResult<TypedExpr<'a>> {
    let (checked_left, checked_right) = (infer(left, env)?, infer(right, env)?);
    match (checked_left.ty(), checked_right.ty()) {
        (TypeValue::Int, TypeValue::Int) => Ok(TypedExpr::new(
            offset,
            TypeValue::Int,
            make(Box::new(checked_left), Box::new(checked_right)),
        )),
        (TypeValue::Float, TypeValue::Float) => Ok(TypedExpr::new(
            offset,
            TypeValue::Float,
            make(Box::new(checked_left), Box::new(checked_right)),
        )),
        (_, _) => todo!(), // TODO - refactor error handling to expect multiple types
    }
}

fn infer_pipe() {
    todo!()
}

fn infer_cmp_binop<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    make: impl FnOnce(Box<TypedExpr<'a>>, Box<TypedExpr<'a>>) -> ExprKind<'a, TypeValue>,
    env: &Env,
) -> TypeResult<TypedExpr<'a>> {
    let (checked_left, checked_right) = (infer(left, env)?, infer(right, env)?);
    match (checked_left.ty(), checked_right.ty()) {
        (TypeValue::Int, TypeValue::Int) | (TypeValue::Float, TypeValue::Float) => {
            Ok(TypedExpr::new(
                offset,
                TypeValue::Bool,
                make(Box::new(checked_left), Box::new(checked_right)),
            ))
        }
        (_, _) => todo!(), // TODO - refactor error handling to expect multiple types
    }
}

fn infer_bool_binop<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    make: impl FnOnce(Box<TypedExpr<'a>>, Box<TypedExpr<'a>>) -> ExprKind<'a, TypeValue>,
    env: &Env,
) -> TypeResult<TypedExpr<'a>> {
    let checked_left = check(left, TypeValue::Bool, env)?;
    let checked_right = check(right, TypeValue::Bool, env)?;
    Ok(TypedExpr::new(
        offset,
        TypeValue::Bool,
        make(Box::new(checked_left), Box::new(checked_right)),
    ))
}

fn infer_if<'a>(
    offset: usize,
    cond: ParsedExpr<'a>,
    thenb: ParsedExpr<'a>,
    elseb: ParsedExpr<'a>,
    env: &Env,
) -> TypeResult<TypedExpr<'a>> {
    let checked_cond = infer(cond, env)?;
    unify(checked_cond.ty(), TypeValue::Bool)?;
    let checked_thenb = infer(thenb, env)?;
    let checked_elseb = infer(elseb, env)?;
    if checked_thenb.ty() != checked_elseb.ty() {
        return Err(TypeError {
            expected: checked_thenb.ty(),
            actual: checked_elseb.ty(),
            offset: checked_elseb.offset,
        });
    }
    Ok(TypedExpr::new(
        offset,
        checked_thenb.ty(),
        ExprKind::If(
            Box::new(checked_cond),
            Box::new(checked_thenb),
            Box::new(checked_elseb),
        ),
    ))
}

fn infer_match<'a>(
    offset: usize,
    expr: ParsedExpr<'a>,
    arms: Vec<(&'a str, Vec<SpannedBinding<'a>>, ParsedExpr<'a>)>,
    env: &Env,
) -> TypeResult<TypedExpr<'a>> {
    todo!()
}
