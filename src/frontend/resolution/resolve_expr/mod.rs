use crate::frontend::{
    parser::ast::{ExprKind, ParsedExpr},
    resolution::types::{ResolvedExpr, ResolverCtx},
};

pub fn resolve_expr(ctx: &mut ResolverCtx, expr: ParsedExpr) -> ResolvedExpr {
    match expr.node.kind {
        ExprKind::Int(val) => ResolvedExpr::Int(val),
        ExprKind::Float(val) => ResolvedExpr::Float(val),
        ExprKind::Bool(val) => ResolvedExpr::Bool(val),
        ExprKind::Id(name) => todo!(),
        ExprKind::Fn(param, body) => todo!(),
        ExprKind::Neg(inner) => ResolvedExpr::Neg(Box::new(resolve_expr(ctx, *inner))),
        ExprKind::FNeg(inner) => ResolvedExpr::FNeg(Box::new(resolve_expr(ctx, *inner))),
        ExprKind::Bang(inner) => ResolvedExpr::Bang(Box::new(resolve_expr(ctx, *inner))),
        ExprKind::Call(func, args) => todo!(),
        ExprKind::Plus(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Plus),
        ExprKind::Minus(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Minus),
        ExprKind::Mult(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Mult),
        ExprKind::Div(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Div),
        ExprKind::FPlus(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::FPlus),
        ExprKind::FMinus(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::FMinus),
        ExprKind::FMult(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::FMult),
        ExprKind::FDiv(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::FDiv),
        ExprKind::Pipe(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Pipe),
        ExprKind::Lt(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Lt),
        ExprKind::LtEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::LtEq),
        ExprKind::Gt(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Gt),
        ExprKind::GtEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::GtEq),
        ExprKind::FLt(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::FLt),
        ExprKind::FLtEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::FLtEq),
        ExprKind::FGt(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::FGt),
        ExprKind::FGtEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::FGtEq),
        ExprKind::Eq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Eq),
        ExprKind::NotEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::NotEq),
        ExprKind::Or(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::Or),
        ExprKind::And(left, right) => resolve_binop(ctx, *left, *right, ResolvedExpr::And),
        ExprKind::If(cond, if_b, then_b) => todo!(),
        ExprKind::Match(expr, arms) => todo!(),
    }
}

pub fn resolve_binop(
    ctx: &mut ResolverCtx,
    l: ParsedExpr,
    r: ParsedExpr,
    make: impl FnOnce(Box<ResolvedExpr>, Box<ResolvedExpr>) -> ResolvedExpr,
) -> ResolvedExpr {
    let l = resolve_expr(ctx, l);
    let r = resolve_expr(ctx, r);
    make(Box::new(l), Box::new(r))
}
