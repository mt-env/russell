use crate::frontend::{
    parser::ast::{ExprKind, ParsedExpr, ParsedMatchArm},
    typechecker::{
        inference::{check::check, context::Context},
        types::{TypeError, TypeResult, TypeValue, TypedExpr},
    },
};

#[cfg(test)]
mod tests;

pub(super) fn infer<'a>(expr: ParsedExpr<'a>, ctx: &mut Context) -> TypeResult<TypedExpr<'a>> {
    let loc = expr.offset;
    match expr.node.kind {
        ExprKind::Int(n) => Ok(TypedExpr::new(loc, TypeValue::Int, ExprKind::Int(n))),
        ExprKind::Float(n) => Ok(TypedExpr::new(loc, TypeValue::Float, ExprKind::Float(n))),
        ExprKind::Bool(val) => Ok(TypedExpr::new(loc, TypeValue::Bool, ExprKind::Bool(val))),
        ExprKind::Id(s) => infer_id(loc, s, ctx),
        ExprKind::Fn(binding, expr) => todo!(),
        ExprKind::Neg(expr) => infer_neg(loc, *expr, ctx),
        ExprKind::FNeg(expr) => infer_fneg(loc, *expr, ctx),
        ExprKind::Bang(expr) => infer_bang(loc, *expr, ctx),
        ExprKind::Call(left, right) => todo!(),
        ExprKind::Plus(left, right) => infer_int_arith(loc, *left, *right, ExprKind::Plus, ctx),
        ExprKind::Minus(left, right) => infer_int_arith(loc, *left, *right, ExprKind::Minus, ctx),
        ExprKind::Mult(left, right) => infer_int_arith(loc, *left, *right, ExprKind::Mult, ctx),
        ExprKind::Div(left, right) => infer_int_arith(loc, *left, *right, ExprKind::Div, ctx),
        ExprKind::FPlus(left, right) => infer_float_arith(loc, *left, *right, ExprKind::FPlus, ctx),
        ExprKind::FMinus(left, right) => {
            infer_float_arith(loc, *left, *right, ExprKind::FMinus, ctx)
        }
        ExprKind::FMult(left, right) => infer_float_arith(loc, *left, *right, ExprKind::FMult, ctx),
        ExprKind::FDiv(left, right) => infer_float_arith(loc, *left, *right, ExprKind::FDiv, ctx),
        ExprKind::Pipe(left, right) => todo!(),
        ExprKind::Lt(left, right) => infer_int_cmp(loc, *left, *right, ExprKind::Lt, ctx),
        ExprKind::LtEq(left, right) => infer_int_cmp(loc, *left, *right, ExprKind::LtEq, ctx),
        ExprKind::Gt(left, right) => infer_int_cmp(loc, *left, *right, ExprKind::Gt, ctx),
        ExprKind::GtEq(left, right) => infer_int_cmp(loc, *left, *right, ExprKind::GtEq, ctx),
        ExprKind::FLt(left, right) => infer_float_cmp(loc, *left, *right, ExprKind::FLt, ctx),
        ExprKind::FLtEq(left, right) => infer_float_cmp(loc, *left, *right, ExprKind::FLtEq, ctx),
        ExprKind::FGt(left, right) => infer_float_cmp(loc, *left, *right, ExprKind::FGt, ctx),
        ExprKind::FGtEq(left, right) => infer_float_cmp(loc, *left, *right, ExprKind::FGtEq, ctx),
        ExprKind::Eq(left, right) => todo!(),
        ExprKind::NotEq(left, right) => todo!(),
        ExprKind::Or(left, right) => infer_bool_binop(loc, *left, *right, ExprKind::Or, ctx),
        ExprKind::And(left, right) => infer_bool_binop(loc, *left, *right, ExprKind::And, ctx),
        ExprKind::If(cond, thenb, elseb) => infer_if(loc, *cond, *thenb, *elseb, ctx),
        ExprKind::Match(expr, arms) => infer_match(loc, *expr, arms, ctx),
    }
}

fn infer_id<'a>(offset: usize, id: &'a str, ctx: &mut Context) -> TypeResult<TypedExpr<'a>> {
    match ctx.lookup(&id) {
        Some(ty) => Ok(TypedExpr::new(offset, ty, ExprKind::Id(id))),
        None => todo!(), // error handling for undefined variable
    }
}

fn infer_fn(binding: String, expr: ParsedExpr, ctx: &mut Context) -> TypeResult<TypeValue> {
    todo!()
}

fn infer_neg<'a>(
    offset: usize,
    expr: ParsedExpr<'a>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let checked_expr = infer(expr, ctx)?;
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

fn infer_fneg<'a>(
    offset: usize,
    expr: ParsedExpr<'a>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let checked_expr = infer(expr, ctx)?;
    ctx.unify(checked_expr.ty(), TypeValue::Float);
    todo!()
}

fn infer_bang<'a>(
    offset: usize,
    expr: ParsedExpr<'a>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let checked_expr = check(expr, TypeValue::Bool, ctx)?;
    Ok(TypedExpr::new(
        offset,
        TypeValue::Bool,
        ExprKind::Bang(Box::new(checked_expr)),
    ))
}

fn infer_call() {
    todo!()
}

fn infer_int_arith<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    make: impl FnOnce(Box<TypedExpr<'a>>, Box<TypedExpr<'a>>) -> ExprKind<'a, TypeValue>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let (checked_left, checked_right) = (infer(left, ctx)?, infer(right, ctx)?);
    match (checked_left.ty(), checked_right.ty()) {
        (TypeValue::Int, TypeValue::Int) => Ok(TypedExpr::new(
            offset,
            TypeValue::Int,
            make(Box::new(checked_left), Box::new(checked_right)),
        )),
        (_, _) => todo!(), // TODO - refactor error handling to expect multiple types
    }
}

fn infer_float_arith<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    make: impl FnOnce(Box<TypedExpr<'a>>, Box<TypedExpr<'a>>) -> ExprKind<'a, TypeValue>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let (checked_left, checked_right) = (infer(left, ctx)?, infer(right, ctx)?);
    match (checked_left.ty(), checked_right.ty()) {
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

fn infer_int_cmp<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    make: impl FnOnce(Box<TypedExpr<'a>>, Box<TypedExpr<'a>>) -> ExprKind<'a, TypeValue>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let (checked_left, checked_right) = (infer(left, ctx)?, infer(right, ctx)?);
    match (checked_left.ty(), checked_right.ty()) {
        (TypeValue::Int, TypeValue::Int) => Ok(TypedExpr::new(
            offset,
            TypeValue::Bool,
            make(Box::new(checked_left), Box::new(checked_right)),
        )),
        (_, _) => todo!(), // TODO - refactor error handling to expect multiple types
    }
}

fn infer_float_cmp<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    make: impl FnOnce(Box<TypedExpr<'a>>, Box<TypedExpr<'a>>) -> ExprKind<'a, TypeValue>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let (checked_left, checked_right) = (infer(left, ctx)?, infer(right, ctx)?);
    match (checked_left.ty(), checked_right.ty()) {
        (TypeValue::Float, TypeValue::Float) => Ok(TypedExpr::new(
            offset,
            TypeValue::Bool,
            make(Box::new(checked_left), Box::new(checked_right)),
        )),
        (_, _) => todo!(), // TODO - refactor error handling to expect multiple types
    }
}

fn infer_bool_binop<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    make: impl FnOnce(Box<TypedExpr<'a>>, Box<TypedExpr<'a>>) -> ExprKind<'a, TypeValue>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let checked_left = check(left, TypeValue::Bool, ctx)?;
    let checked_right = check(right, TypeValue::Bool, ctx)?;
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
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    let checked_cond = infer(cond, ctx)?;
    ctx.unify(checked_cond.ty(), TypeValue::Bool);
    let checked_thenb = infer(thenb, ctx)?;
    let checked_elseb = infer(elseb, ctx)?;
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
    arms: Vec<ParsedMatchArm<'a>>,
    ctx: &mut Context,
) -> TypeResult<TypedExpr<'a>> {
    todo!()
}
