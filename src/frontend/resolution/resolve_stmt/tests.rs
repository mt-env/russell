use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_stmt::parse_stmt},
    resolution::{
        resolve_stmt,
        types::{
            ResolutionErrorKind, ResolvedExpr, ResolvedExprKind, ResolvedStmt, ResolvedType,
            ResolverCtx,
        },
    },
    types::Spanned,
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedStmt {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    let parsed = parse_stmt(&mut parser).unwrap();
    resolve_stmt::resolve_stmt(ctx, parsed)
}

fn expr(offset: usize, id: usize, kind: ResolvedExprKind) -> ResolvedExpr {
    let mut ctx = ResolverCtx::new();
    let mut expr_id = ctx.next_exprid();
    for _ in 0..id {
        expr_id = ctx.next_exprid();
    }
    ResolvedExpr::new(offset, expr_id, kind)
}

#[test]
fn resolves_let_statement_and_binds_name() {
    let mut ctx = ResolverCtx::new();
    let resolved = resolve(&mut ctx, "let x = 7;");
    let x = ctx.lookup_value("x").expect("x should be bound");

    assert_eq!(
        resolved,
        ResolvedStmt::mk_let(0, x, expr(8, 0, ResolvedExprKind::Int(7)))
    );
}

#[test]
fn resolves_let_statement_with_identifier_rhs() {
    let mut ctx = ResolverCtx::new();
    let y = ctx.add_value("y");
    let resolved = resolve(&mut ctx, "let x = y;");
    let x = ctx.lookup_value("x").expect("x should be bound");

    assert_eq!(
        resolved,
        ResolvedStmt::mk_let(0, x, expr(8, 0, ResolvedExprKind::Id(y)))
    );
}

#[test]
fn resolves_let_statement_with_shadowing() {
    let mut ctx = ResolverCtx::new();
    let old = ctx.add_value("x");
    let resolved = resolve(&mut ctx, "let x = 10;");
    let new = ctx.lookup_value("x").expect("x should be rebound");

    assert_ne!(new, old);
    assert_eq!(
        resolved,
        ResolvedStmt::mk_let(0, new, expr(8, 0, ResolvedExprKind::Int(10)))
    );
}

#[test]
fn resolves_return_identifier_using_existing_binding() {
    let mut ctx = ResolverCtx::new();
    let expected = ctx.add_value("x");
    let resolved = resolve(&mut ctx, "return x;");
    assert_eq!(
        resolved,
        ResolvedStmt::mk_return(0, expr(7, 0, ResolvedExprKind::Id(expected)))
    );
}

#[test]
fn resolves_return_literal_expression() {
    let resolved = resolve(&mut ResolverCtx::new(), "return 99;");
    assert_eq!(
        resolved,
        ResolvedStmt::mk_return(0, expr(7, 0, ResolvedExprKind::Int(99)))
    );
}

#[test]
fn resolves_return_binary_expression() {
    let resolved = resolve(&mut ResolverCtx::new(), "return 1 + 2;");
    assert_eq!(
        resolved,
        ResolvedStmt::mk_return(
            0,
            expr(
                7,
                2,
                ResolvedExprKind::Plus(
                    Box::new(expr(7, 0, ResolvedExprKind::Int(1))),
                    Box::new(expr(11, 1, ResolvedExprKind::Int(2)))
                )
            )
        )
    );
}

#[test]
fn resolves_read_statement() {
    let mut ctx = ResolverCtx::new();
    let resolved = resolve(&mut ctx, "read Int x;");
    let x = ctx.lookup_value("x").expect("x should be bound");

    assert_eq!(
        resolved,
        ResolvedStmt::mk_read(0, x, ResolvedType::mk_int(5))
    );
}

#[test]
fn resolves_echo_statement() {
    let ctx = &mut ResolverCtx::new();
    let resolved = resolve(ctx, "echo Int 42;");
    assert_eq!(
        resolved,
        ResolvedStmt::mk_echo(
            0,
            ResolvedType::mk_int(5),
            expr(9, 0, ResolvedExprKind::Int(42))
        )
    );
}

#[test]
fn return_statement_panics_on_unbound_identifier() {
    let mut ctx = ResolverCtx::new();
    let _ = resolve(&mut ctx, "return missing;");
    assert_eq!(
        ctx.errors,
        vec![Spanned {
            offset: 7,
            node: ResolutionErrorKind::UnknownValue { name: "missing" },
        }]
    );
}
