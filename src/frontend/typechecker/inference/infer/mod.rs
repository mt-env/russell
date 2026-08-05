use crate::frontend::{
    parser::ast::{ExprKind, ParsedExpr, ParsedMatchArm},
    typechecker::{
        inference::{
            check::check,
            context::{Context, InferredExpr, TypeId},
        },
        types::{TypeResult, TypeValue},
    },
};

#[cfg(test)]
mod tests;

pub(super) fn infer<'a>(expr: ParsedExpr<'a>, ctx: &mut Context) -> TypeResult<InferredExpr<'a>> {
    let loc = expr.offset;
    match expr.node.kind {
        ExprKind::Int(n) => Ok(InferredExpr::new(loc, ctx.int_id(), ExprKind::Int(n))),
        ExprKind::Float(n) => Ok(InferredExpr::new(loc, ctx.float_id(), ExprKind::Float(n))),
        ExprKind::Bool(val) => Ok(InferredExpr::new(loc, ctx.bool_id(), ExprKind::Bool(val))),
        ExprKind::Id(s) => infer_id(loc, s, ctx),
        ExprKind::Fn(binding, expr) => todo!(),
        ExprKind::Neg(expr) => infer_unop(loc, *expr, ctx, ExprKind::Neg, ctx.int_id()),
        ExprKind::FNeg(expr) => infer_unop(loc, *expr, ctx, ExprKind::FNeg, ctx.float_id()),
        ExprKind::Bang(expr) => infer_unop(loc, *expr, ctx, ExprKind::Bang, ctx.bool_id()),
        ExprKind::Call(left, right) => todo!(),
        ExprKind::Plus(l, r) => infer_int_arith(loc, *l, *r, ctx, ExprKind::Plus),
        ExprKind::Minus(l, r) => infer_int_arith(loc, *l, *r, ctx, ExprKind::Minus),
        ExprKind::Mult(l, r) => infer_int_arith(loc, *l, *r, ctx, ExprKind::Mult),
        ExprKind::Div(l, r) => infer_int_arith(loc, *l, *r, ctx, ExprKind::Div),
        ExprKind::FPlus(l, r) => infer_float_arith(loc, *l, *r, ctx, ExprKind::FPlus),
        ExprKind::FMinus(l, r) => infer_float_arith(loc, *l, *r, ctx, ExprKind::FMinus),
        ExprKind::FMult(l, r) => infer_float_arith(loc, *l, *r, ctx, ExprKind::FMult),
        ExprKind::FDiv(l, r) => infer_float_arith(loc, *l, *r, ctx, ExprKind::FDiv),
        ExprKind::Pipe(l, r) => todo!(),
        ExprKind::Lt(l, r) => infer_int_cmp(loc, *l, *r, ctx, ExprKind::Lt),
        ExprKind::LtEq(l, r) => infer_int_cmp(loc, *l, *r, ctx, ExprKind::LtEq),
        ExprKind::Gt(l, r) => infer_int_cmp(loc, *l, *r, ctx, ExprKind::Gt),
        ExprKind::GtEq(l, r) => infer_int_cmp(loc, *l, *r, ctx, ExprKind::GtEq),
        ExprKind::FLt(l, r) => infer_float_cmp(loc, *l, *r, ctx, ExprKind::FLt),
        ExprKind::FLtEq(l, r) => infer_float_cmp(loc, *l, *r, ctx, ExprKind::FLtEq),
        ExprKind::FGt(l, r) => infer_float_cmp(loc, *l, *r, ctx, ExprKind::FGt),
        ExprKind::FGtEq(l, r) => infer_float_cmp(loc, *l, *r, ctx, ExprKind::FGtEq),
        ExprKind::Eq(l, r) => infer_eq(loc, *l, *r, ctx, ExprKind::Eq),
        ExprKind::NotEq(l, r) => infer_eq(loc, *l, *r, ctx, ExprKind::NotEq),
        ExprKind::Or(l, r) => infer_bool_binop(loc, *l, *r, ctx, ExprKind::Or),
        ExprKind::And(l, r) => infer_bool_binop(loc, *l, *r, ctx, ExprKind::And),
        ExprKind::If(cond, thenb, elseb) => infer_if(loc, *cond, *thenb, *elseb, ctx),
        ExprKind::Match(expr, arms) => infer_match(loc, *expr, arms, ctx),
    }
}

fn infer_id<'a>(offset: usize, id: &'a str, ctx: &mut Context) -> TypeResult<InferredExpr<'a>> {
    todo!()
}

fn infer_fn(binding: String, expr: ParsedExpr, ctx: &mut Context) -> TypeResult<TypeValue> {
    todo!()
}

fn infer_unop<'a>(
    offset: usize,
    expr: ParsedExpr<'a>,
    ctx: &mut Context,
    expr_kind: impl Fn(Box<InferredExpr<'a>>) -> ExprKind<'a, TypeId>,
    expected_type: TypeId,
) -> TypeResult<InferredExpr<'a>> {
    let checked_expr = check(expr, expected_type, ctx)?;
    Ok(InferredExpr::new(
        offset,
        expected_type,
        expr_kind(Box::new(checked_expr)),
    ))
}

fn infer_call() {
    todo!()
}

fn infer_binop<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    ctx: &mut Context,
    expr_kind: impl Fn(Box<InferredExpr<'a>>, Box<InferredExpr<'a>>) -> ExprKind<'a, TypeId>,
    operand_ty: TypeId,
    output_ty: TypeId,
) -> TypeResult<InferredExpr<'a>> {
    let left = check(left, operand_ty, ctx)?;
    let right = check(right, operand_ty, ctx)?;
    Ok(InferredExpr::new(
        offset,
        output_ty,
        expr_kind(Box::new(left), Box::new(right)),
    ))
}

fn infer_int_arith<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    ctx: &mut Context,
    expr_kind: impl Fn(Box<InferredExpr<'a>>, Box<InferredExpr<'a>>) -> ExprKind<'a, TypeId>,
) -> TypeResult<InferredExpr<'a>> {
    infer_binop(
        offset,
        left,
        right,
        ctx,
        expr_kind,
        ctx.int_id(),
        ctx.int_id(),
    )
}

fn infer_float_arith<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    ctx: &mut Context,
    expr_kind: impl Fn(Box<InferredExpr<'a>>, Box<InferredExpr<'a>>) -> ExprKind<'a, TypeId>,
) -> TypeResult<InferredExpr<'a>> {
    infer_binop(
        offset,
        left,
        right,
        ctx,
        expr_kind,
        ctx.float_id(),
        ctx.float_id(),
    )
}

fn infer_int_cmp<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    ctx: &mut Context,
    expr_kind: impl Fn(Box<InferredExpr<'a>>, Box<InferredExpr<'a>>) -> ExprKind<'a, TypeId>,
) -> TypeResult<InferredExpr<'a>> {
    infer_binop(
        offset,
        left,
        right,
        ctx,
        expr_kind,
        ctx.int_id(),
        ctx.bool_id(),
    )
}

fn infer_float_cmp<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    ctx: &mut Context,
    expr_kind: impl Fn(Box<InferredExpr<'a>>, Box<InferredExpr<'a>>) -> ExprKind<'a, TypeId>,
) -> TypeResult<InferredExpr<'a>> {
    infer_binop(
        offset,
        left,
        right,
        ctx,
        expr_kind,
        ctx.float_id(),
        ctx.bool_id(),
    )
}

fn infer_bool_binop<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    ctx: &mut Context,
    expr_kind: impl Fn(Box<InferredExpr<'a>>, Box<InferredExpr<'a>>) -> ExprKind<'a, TypeId>,
) -> TypeResult<InferredExpr<'a>> {
    infer_binop(
        offset,
        left,
        right,
        ctx,
        expr_kind,
        ctx.bool_id(),
        ctx.bool_id(),
    )
}

fn infer_eq<'a>(
    offset: usize,
    left: ParsedExpr<'a>,
    right: ParsedExpr<'a>,
    ctx: &mut Context,
    expr_kind: impl Fn(Box<InferredExpr<'a>>, Box<InferredExpr<'a>>) -> ExprKind<'a, TypeId>,
) -> TypeResult<InferredExpr<'a>> {
    let operand_ty = ctx.new_tyvar();
    infer_binop(
        offset,
        left,
        right,
        ctx,
        expr_kind,
        operand_ty,
        ctx.bool_id(),
    )
}

fn infer_pipe() {
    todo!()
}

fn infer_if<'a>(
    offset: usize,
    cond: ParsedExpr<'a>,
    thenb: ParsedExpr<'a>,
    elseb: ParsedExpr<'a>,
    ctx: &mut Context,
) -> TypeResult<InferredExpr<'a>> {
    let checked_cond = check(cond, ctx.bool_id(), ctx)?;
    let output_type = ctx.new_tyvar();
    let checked_thenb = check(thenb, output_type, ctx)?;
    let checked_elseb = check(elseb, output_type, ctx)?;
    Ok(InferredExpr::new(
        offset,
        output_type,
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
) -> TypeResult<InferredExpr<'a>> {
    todo!()
}
