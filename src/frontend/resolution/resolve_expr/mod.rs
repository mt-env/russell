use crate::frontend::{
    parser::ast::{ExprKind, ParsedBinding, ParsedExpr, ParsedMatchArm},
    resolution::{
        resolve_type,
        types::{ResolvedExprKind, ResolvedMatchArm, ResolverCtx},
    },
};

#[cfg(test)]
mod tests;

pub fn resolve_expr<'a>(ctx: &mut ResolverCtx<'a>, expr: ParsedExpr<'a>) -> ResolvedExprKind {
    match expr.node.kind {
        ExprKind::Int(val) => ResolvedExprKind::Int(val),
        ExprKind::Float(val) => ResolvedExprKind::Float(val),
        ExprKind::Bool(val) => ResolvedExprKind::Bool(val),
        ExprKind::Id(name) => resolve_id(ctx, name),
        ExprKind::Fn(param, body) => resolve_closure(ctx, param, *body),
        ExprKind::Neg(inner) => ResolvedExprKind::Neg(Box::new(resolve_expr(ctx, *inner))),
        ExprKind::FNeg(inner) => ResolvedExprKind::FNeg(Box::new(resolve_expr(ctx, *inner))),
        ExprKind::Bang(inner) => ResolvedExprKind::Bang(Box::new(resolve_expr(ctx, *inner))),
        ExprKind::Call(func, args) => resolve_call(ctx, *func, args),
        ExprKind::Plus(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Plus),
        ExprKind::Minus(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Minus),
        ExprKind::Mult(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Mult),
        ExprKind::Div(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Div),
        ExprKind::FPlus(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::FPlus),
        ExprKind::FMinus(left, right) => {
            resolve_binop(ctx, *left, *right, ResolvedExprKind::FMinus)
        }
        ExprKind::FMult(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::FMult),
        ExprKind::FDiv(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::FDiv),
        ExprKind::Pipe(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Pipe),
        ExprKind::Lt(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Lt),
        ExprKind::LtEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::LtEq),
        ExprKind::Gt(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Gt),
        ExprKind::GtEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::GtEq),
        ExprKind::FLt(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::FLt),
        ExprKind::FLtEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::FLtEq),
        ExprKind::FGt(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::FGt),
        ExprKind::FGtEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::FGtEq),
        ExprKind::Eq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Eq),
        ExprKind::NotEq(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::NotEq),
        ExprKind::Or(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::Or),
        ExprKind::And(left, right) => resolve_binop(ctx, *left, *right, ResolvedExprKind::And),
        ExprKind::If(cond, if_b, then_b) => resolve_if(ctx, *cond, *if_b, *then_b),
        ExprKind::Match(expr, arms) => resolve_match(ctx, *expr, arms),
    }
}

fn resolve_id<'a>(ctx: &ResolverCtx<'a>, name: &'a str) -> ResolvedExprKind {
    let Some(value_id) = ctx.lookup_value(name) else {
        // TODO - don't panic, allow for error recovery
        panic!("{}: unbound or not a value", name);
    };
    ResolvedExprKind::Id(value_id)
}

fn resolve_closure<'a>(
    ctx: &mut ResolverCtx<'a>,
    param: ParsedBinding<'a>,
    body: ParsedExpr<'a>,
) -> ResolvedExprKind {
    ctx.push_scope();
    let binding = resolve_type::add_binding(ctx, param);
    let body = resolve_expr(ctx, body);
    ctx.pop_scope();
    ResolvedExprKind::Fn(binding, Box::new(body))
}

fn resolve_call<'a>(
    ctx: &mut ResolverCtx<'a>,
    func: ParsedExpr<'a>,
    args: Vec<ParsedExpr<'a>>,
) -> ResolvedExprKind {
    let func = resolve_expr(ctx, func);
    let args = args
        .into_iter()
        .map(|arg| resolve_expr(ctx, arg))
        .collect::<Vec<_>>();
    ResolvedExprKind::Call(Box::new(func), args)
}

fn resolve_binop<'a>(
    ctx: &mut ResolverCtx<'a>,
    l: ParsedExpr<'a>,
    r: ParsedExpr<'a>,
    make: impl Fn(Box<ResolvedExprKind>, Box<ResolvedExprKind>) -> ResolvedExprKind,
) -> ResolvedExprKind {
    let l = resolve_expr(ctx, l);
    let r = resolve_expr(ctx, r);
    make(Box::new(l), Box::new(r))
}

fn resolve_if<'a>(
    ctx: &mut ResolverCtx<'a>,
    cond: ParsedExpr<'a>,
    if_b: ParsedExpr<'a>,
    then_b: ParsedExpr<'a>,
) -> ResolvedExprKind {
    let cond = resolve_expr(ctx, cond);
    let if_b = resolve_expr(ctx, if_b);
    let then_b = resolve_expr(ctx, then_b);
    ResolvedExprKind::If(Box::new(cond), Box::new(if_b), Box::new(then_b))
}

fn resolve_match<'a>(
    ctx: &mut ResolverCtx<'a>,
    expr: ParsedExpr<'a>,
    arms: Vec<ParsedMatchArm<'a>>,
) -> ResolvedExprKind {
    let expr = resolve_expr(ctx, expr);
    let mut resolved_arms = Vec::new();

    for arm in arms {
        let arm = arm.node;

        // find the variant
        let Some(variant) = ctx.lookup_value(arm.id) else {
            panic!("cannot find variant {}", arm.id);
        };

        // bind the pattern variables in a new scope and resolve the body
        ctx.push_scope();
        let mut binding_ids = Vec::new();
        for binding in arm.bindings {
            binding_ids.push(ctx.add_value(binding));
        }
        let resolved_body = resolve_expr(ctx, arm.expr);
        ctx.pop_scope();

        let resolved_arm = ResolvedMatchArm::new(variant, binding_ids, resolved_body);

        resolved_arms.push(resolved_arm);
    }

    ResolvedExprKind::Match(Box::new(expr), resolved_arms)
}
