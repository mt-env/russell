use crate::frontend::{
    parser::ast::{ParsedExpr, ParsedStmt, ParsedType, Stmt},
    resolution::{
        resolve_expr, resolve_type,
        types::{ResolvedStmt, ResolverCtx},
    },
};

#[cfg(test)]
mod tests;

pub fn resolve_stmt<'src>(ctx: &mut ResolverCtx<'src>, stmt: ParsedStmt<'src>) -> ResolvedStmt {
    let loc = stmt.offset;
    match stmt.node {
        Stmt::Let(id, expr) => resolve_let(loc, ctx, id, expr),
        Stmt::Read(ty, id) => resolve_read(loc, ctx, ty, id),
        Stmt::Echo(ty, expr) => resolve_echo(loc, ctx, ty, expr),
        Stmt::Return(expr) => resolve_return(loc, ctx, expr),
    }
}

fn resolve_let<'src>(
    offset: usize,
    ctx: &mut ResolverCtx<'src>,
    id: &'src str,
    expr: ParsedExpr<'src>,
) -> ResolvedStmt {
    let resolved_expr = resolve_expr::resolve_expr(ctx, expr);
    let value_id = ctx.add_value(id);
    ResolvedStmt::mk_let(offset, value_id, resolved_expr)
}

fn resolve_read<'src>(
    offset: usize,
    ctx: &mut ResolverCtx<'src>,
    ty: ParsedType<'src>,
    id: &'src str,
) -> ResolvedStmt {
    let value_id = ctx.add_value(id);
    ResolvedStmt::mk_read(offset, value_id, resolve_type::add_type(ctx, ty))
}

fn resolve_echo<'src>(
    offset: usize,
    ctx: &mut ResolverCtx<'src>,
    ty: ParsedType<'src>,
    expr: ParsedExpr<'src>,
) -> ResolvedStmt {
    // todo should this use add_type or not?
    ResolvedStmt::mk_echo(
        offset,
        resolve_type::add_type(ctx, ty),
        resolve_expr::resolve_expr(ctx, expr),
    )
}

fn resolve_return<'src>(
    offset: usize,
    ctx: &mut ResolverCtx<'src>,
    expr: ParsedExpr<'src>,
) -> ResolvedStmt {
    ResolvedStmt::mk_return(offset, resolve_expr::resolve_expr(ctx, expr))
}
