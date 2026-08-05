use crate::frontend::{
    parser::ast::{ParsedExpr, ParsedStmt, Stmt, Type},
    resolution::{
        resolve_expr, resolve_type,
        types::{ResolvedStmt, ResolverCtx},
    },
};

#[cfg(test)]
mod tests;

pub fn resolve_stmt<'a>(ctx: &mut ResolverCtx<'a>, stmt: ParsedStmt<'a>) -> ResolvedStmt {
    // TODO - retain source location information for error reporting
    match stmt.node {
        Stmt::Let(id, expr) => resolve_let(ctx, id, expr),
        Stmt::Read(ty, id) => resolve_read(ctx, ty, id),
        Stmt::Echo(ty, expr) => resolve_echo(ctx, ty, expr),
        Stmt::Return(expr) => resolve_return(ctx, expr),
    }
}

fn resolve_let<'a>(ctx: &mut ResolverCtx<'a>, id: &'a str, expr: ParsedExpr<'a>) -> ResolvedStmt {
    let resolved_expr = resolve_expr::resolve_expr(ctx, expr);
    let value_id = ctx.add_value(id);
    ResolvedStmt::Let(value_id, resolved_expr)
}

fn resolve_read<'a>(ctx: &mut ResolverCtx<'a>, ty: Type<'a>, id: &'a str) -> ResolvedStmt {
    let value_id = ctx.add_value(id);
    ResolvedStmt::Read(value_id, resolve_type::add_type(ctx, ty))
}

fn resolve_echo<'a>(ctx: &mut ResolverCtx<'a>, ty: Type<'a>, expr: ParsedExpr<'a>) -> ResolvedStmt {
    ResolvedStmt::Echo(
        resolve_type::add_type(ctx, ty),
        resolve_expr::resolve_expr(ctx, expr),
    )
}

fn resolve_return<'a>(ctx: &mut ResolverCtx<'a>, expr: ParsedExpr<'a>) -> ResolvedStmt {
    ResolvedStmt::Return(resolve_expr::resolve_expr(ctx, expr))
}
