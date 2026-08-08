use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_stmt::parse_stmt},
    resolution::{
        resolve_stmt,
        types::{ResolvedExpr, ResolvedStmt, ResolvedType, ResolverCtx},
    },
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedStmt {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    let parsed = parse_stmt(&mut parser).unwrap();
    resolve_stmt::resolve_stmt(ctx, parsed)
}

#[test]
fn resolves_let_statement_and_binds_name() {
    let mut ctx = ResolverCtx::new();
    let resolved = resolve(&mut ctx, "let x = 7;");
    let x = ctx.lookup_value("x").expect("x should be bound");

    assert_eq!(resolved, ResolvedStmt::Let(x, ResolvedExpr::Int(7)));
}

#[test]
fn resolves_let_statement_with_identifier_rhs() {
    let mut ctx = ResolverCtx::new();
    let y = ctx.add_value("y");
    let resolved = resolve(&mut ctx, "let x = y;");
    let x = ctx.lookup_value("x").expect("x should be bound");

    assert_eq!(resolved, ResolvedStmt::Let(x, ResolvedExpr::Id(y)));
}

#[test]
fn resolves_let_statement_with_shadowing() {
    let mut ctx = ResolverCtx::new();
    let old = ctx.add_value("x");
    let resolved = resolve(&mut ctx, "let x = 10;");
    let new = ctx.lookup_value("x").expect("x should be rebound");

    assert_ne!(new, old);
    assert_eq!(resolved, ResolvedStmt::Let(new, ResolvedExpr::Int(10)));
}

#[test]
fn resolves_return_identifier_using_existing_binding() {
    let mut ctx = ResolverCtx::new();
    let expected = ctx.add_value("x");
    let resolved = resolve(&mut ctx, "return x;");
    assert_eq!(resolved, ResolvedStmt::Return(ResolvedExpr::Id(expected)));
}

#[test]
fn resolves_return_literal_expression() {
    let resolved = resolve(&mut ResolverCtx::new(), "return 99;");
    assert_eq!(resolved, ResolvedStmt::Return(ResolvedExpr::Int(99)));
}

#[test]
fn resolves_return_binary_expression() {
    let resolved = resolve(&mut ResolverCtx::new(), "return 1 + 2;");
    assert_eq!(
        resolved,
        ResolvedStmt::Return(ResolvedExpr::Plus(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2))
        ))
    );
}

#[test]
fn resolves_read_statement() {
    let mut ctx = ResolverCtx::new();
    let resolved = resolve(&mut ctx, "read Int x;");
    let x = ctx.lookup_value("x").expect("x should be bound");

    assert_eq!(resolved, ResolvedStmt::Read(x, ResolvedType::Int));
}

#[test]
fn resolves_echo_statement() {
    let ctx = &mut ResolverCtx::new();
    let resolved = resolve(ctx, "echo Int 42;");
    assert_eq!(
        resolved,
        ResolvedStmt::Echo(ResolvedType::Int, ResolvedExpr::Int(42))
    );
}

#[test]
#[should_panic(expected = "unbound")]
fn return_statement_panics_on_unbound_identifier() {
    // TODO - this shouldn't panic in the future, we should use Result
    let _ = resolve(&mut ResolverCtx::new(), "return missing;");
}
