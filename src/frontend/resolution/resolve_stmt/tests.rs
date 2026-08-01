use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_stmt::parse_stmt},
    resolution::{
        resolve_stmt,
        types::{ResolvedExpr, ResolvedStmt, ResolvedType, ResolverCtx},
    },
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedStmt {
    let mut p = Parser::new(lex(input));
    let parsed_stmt = parse_stmt(&mut p).unwrap();
    resolve_stmt::resolve_stmt(ctx, parsed_stmt)
}

#[test]
fn resolves_let_statement_and_binds_name() {
    let ctx = &mut ResolverCtx::new();
    let resolved = resolve(ctx, "let x = 7;");
    let id = ctx.lookup_value("x");
    assert!(id.is_some(), "expected 'x' to be bound as a value");
    assert_eq!(
        resolved,
        ResolvedStmt::Let(id.unwrap(), ResolvedExpr::Int(7))
    );
}

#[test]
fn resolves_let_statement_with_identifier_rhs() {
    let ctx = &mut ResolverCtx::new();
    let y_id = ctx.add_value("y");
    let resolved = resolve(ctx, "let x = y;");
    let x_id = ctx.lookup_value("x");
    assert!(x_id.is_some(), "expected 'x' to be bound as a value");
    assert_eq!(
        resolved,
        ResolvedStmt::Let(x_id.unwrap(), ResolvedExpr::Id(y_id))
    );
}

#[test]
fn resolves_let_statement_with_shadowing() {
    let ctx = &mut ResolverCtx::new();
    let old = ctx.add_value("x");
    let resolved = resolve(ctx, "let x = 10;");
    let new = ctx.lookup_value("x");
    assert!(new.is_some(), "expected 'x' to be bound as a value");
    assert!(new.unwrap() != old);
    assert_eq!(
        resolved,
        ResolvedStmt::Let(new.unwrap(), ResolvedExpr::Int(10))
    );
}

#[test]
fn resolves_return_identifier_using_existing_binding() {
    let ctx = &mut ResolverCtx::new();
    let expected = ctx.add_value("x");
    let resolved = resolve(ctx, "return x;");
    assert_eq!(resolved, ResolvedStmt::Return(ResolvedExpr::Id(expected)));
}

#[test]
fn resolves_return_literal_expression() {
    let resolved = resolve(&mut ResolverCtx::new(), "return 99;");
    assert_eq!(resolved, ResolvedStmt::Return(ResolvedExpr::Int(99)));
}

#[test]
fn resolves_return_binary_expression() {
    let ctx = &mut ResolverCtx::new();
    let resolved = resolve(ctx, "return 1 + 2;");
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
    let ctx = &mut ResolverCtx::new();
    let resolved = resolve(ctx, "read Int x;");
    let x_id = ctx.lookup_value("x");
    assert!(x_id.is_some(), "expected 'x' to be bound as a value");
    assert_eq!(
        resolved,
        ResolvedStmt::Read(x_id.unwrap(), ResolvedType::Int)
    );
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
