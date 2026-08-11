use crate::frontend::{
    parser::ast::{ExprKind, ParsedBinding, ParsedExpr, ParsedMatchArm},
    resolution::{
        resolve_type,
        types::{ResolvedExpr, ResolvedExprKind, ResolvedMatchArm, ResolverCtx},
    },
};

#[cfg(test)]
mod tests;

pub fn resolve_expr<'a>(ctx: &mut ResolverCtx<'a>, expr: ParsedExpr<'a>) -> ResolvedExpr {
    let loc = expr.offset;
    match expr.node.kind {
        ExprKind::Int(val) => resolve_primitive(loc, ctx, val, ResolvedExprKind::Int),
        ExprKind::Float(val) => resolve_primitive(loc, ctx, val, ResolvedExprKind::Float),
        ExprKind::Bool(val) => resolve_primitive(loc, ctx, val, ResolvedExprKind::Bool),
        ExprKind::Id(name) => resolve_id(loc, ctx, name),
        ExprKind::Fn(param, body) => resolve_closure(loc, ctx, param, *body),
        ExprKind::Neg(inner) => resolve_unop(loc, ctx, *inner, ResolvedExprKind::Neg),
        ExprKind::FNeg(inner) => resolve_unop(loc, ctx, *inner, ResolvedExprKind::FNeg),
        ExprKind::Bang(inner) => resolve_unop(loc, ctx, *inner, ResolvedExprKind::Bang),
        ExprKind::Call(func, args) => resolve_call(loc, ctx, *func, args),
        ExprKind::Plus(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Plus),
        ExprKind::Minus(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Minus),
        ExprKind::Mult(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Mult),
        ExprKind::Div(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Div),
        ExprKind::FPlus(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::FPlus),
        ExprKind::FMinus(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::FMinus),
        ExprKind::FMult(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::FMult),
        ExprKind::FDiv(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::FDiv),
        ExprKind::Pipe(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Pipe),
        ExprKind::Lt(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Lt),
        ExprKind::LtEq(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::LtEq),
        ExprKind::Gt(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Gt),
        ExprKind::GtEq(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::GtEq),
        ExprKind::FLt(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::FLt),
        ExprKind::FLtEq(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::FLtEq),
        ExprKind::FGt(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::FGt),
        ExprKind::FGtEq(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::FGtEq),
        ExprKind::Eq(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Eq),
        ExprKind::NotEq(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::NotEq),
        ExprKind::Or(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::Or),
        ExprKind::And(l, r) => resolve_binop(loc, ctx, *l, *r, ResolvedExprKind::And),
        ExprKind::If(cond, if_b, then_b) => resolve_if(loc, ctx, *cond, *if_b, *then_b),
        ExprKind::Match(expr, arms) => resolve_match(loc, ctx, *expr, arms),
    }
}

fn resolve_primitive<A>(
    offset: usize,
    ctx: &mut ResolverCtx,
    val: A,
    make: impl Fn(A) -> ResolvedExprKind,
) -> ResolvedExpr {
    let id = ctx.next_exprid();
    ResolvedExpr::new(offset, id, make(val))
}

fn resolve_unop<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    inner: ParsedExpr<'a>,
    make: impl Fn(Box<ResolvedExpr>) -> ResolvedExprKind,
) -> ResolvedExpr {
    let resolved_inner = resolve_expr(ctx, inner);
    let id = ctx.next_exprid();
    ResolvedExpr::new(offset, id, make(Box::new(resolved_inner)))
}

fn resolve_id<'a>(offset: usize, ctx: &mut ResolverCtx<'a>, name: &'a str) -> ResolvedExpr {
    let id = ctx.next_exprid();
    let value_id = match ctx.lookup_value(name) {
        Some(value_id) => value_id,
        None => {
            ctx.add_unbound_val_error(name, offset);
            ctx.add_value(name) // add a placeholder to continue
        }
    };

    ResolvedExpr::new(offset, id, ResolvedExprKind::Id(value_id))
}

fn resolve_closure<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    param: ParsedBinding<'a>,
    body: ParsedExpr<'a>,
) -> ResolvedExpr {
    ctx.push_scope();
    let binding = resolve_type::add_binding(ctx, param);
    let body = resolve_expr(ctx, body);
    ctx.pop_scope();
    let id = ctx.next_exprid();
    ResolvedExpr::new(offset, id, ResolvedExprKind::Fn(binding, Box::new(body)))
}

fn resolve_call<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    func: ParsedExpr<'a>,
    args: Vec<ParsedExpr<'a>>,
) -> ResolvedExpr {
    let func = resolve_expr(ctx, func);
    let args = args
        .into_iter()
        .map(|arg| resolve_expr(ctx, arg))
        .collect::<Vec<_>>();
    let id = ctx.next_exprid();
    ResolvedExpr::new(offset, id, ResolvedExprKind::Call(Box::new(func), args))
}

fn resolve_binop<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    l: ParsedExpr<'a>,
    r: ParsedExpr<'a>,
    make: impl Fn(Box<ResolvedExpr>, Box<ResolvedExpr>) -> ResolvedExprKind,
) -> ResolvedExpr {
    let l = resolve_expr(ctx, l);
    let r = resolve_expr(ctx, r);
    let id = ctx.next_exprid();
    ResolvedExpr::new(offset, id, make(Box::new(l), Box::new(r)))
}

fn resolve_if<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    cond: ParsedExpr<'a>,
    if_b: ParsedExpr<'a>,
    then_b: ParsedExpr<'a>,
) -> ResolvedExpr {
    let cond = resolve_expr(ctx, cond);
    let if_b = resolve_expr(ctx, if_b);
    let then_b = resolve_expr(ctx, then_b);
    let id = ctx.next_exprid();
    ResolvedExpr::new(
        offset,
        id,
        ResolvedExprKind::If(Box::new(cond), Box::new(if_b), Box::new(then_b)),
    )
}

fn resolve_match<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    expr: ParsedExpr<'a>,
    arms: Vec<ParsedMatchArm<'a>>,
) -> ResolvedExpr {
    let expr = resolve_expr(ctx, expr);
    let mut resolved_arms = Vec::new();

    for arm in arms {
        let arm_data = arm.node;

        let variant = match ctx.lookup_value(arm_data.id) {
            Some(variant) => variant,
            None => {
                ctx.add_unbound_val_error(arm_data.id, arm.offset);
                ctx.add_value(arm_data.id) // add a placeholder to continue
            }
        };

        // bind the pattern variables in a new scope and resolve the body
        ctx.push_scope();
        let mut binding_ids = Vec::new();
        for binding in arm_data.bindings {
            binding_ids.push(ctx.add_value(binding));
        }
        let resolved_body = resolve_expr(ctx, arm_data.expr);
        ctx.pop_scope();

        let resolved_arm = ResolvedMatchArm::new(arm.offset, variant, binding_ids, resolved_body);

        resolved_arms.push(resolved_arm);
    }

    let id = ctx.next_exprid();

    ResolvedExpr::new(
        offset,
        id,
        ResolvedExprKind::Match(Box::new(expr), resolved_arms),
    )
}
