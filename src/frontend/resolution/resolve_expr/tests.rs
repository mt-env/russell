use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_expr::parse_expr},
    resolution::types::{
        ResolvedBinding, ResolvedExpr, ResolvedMatchArm, ResolvedType, ResolverCtx,
    },
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedExpr {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    let parsed = parse_expr(&mut parser).unwrap();
    super::resolve_expr(ctx, parsed)
}

#[test]
fn resolves_literals() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "42"),
        ResolvedExpr::Int(42)
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "3.5"),
        ResolvedExpr::Float(3.5)
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true"),
        ResolvedExpr::Bool(true)
    );
}

#[test]
fn resolves_identifier_from_context() {
    let mut ctx = ResolverCtx::new();
    let x = ctx.add_value("x");

    assert_eq!(resolve(&mut ctx, "x"), ResolvedExpr::Id(x));
}

#[test]
#[should_panic(expected = "missing: unbound or not a value")]
fn rejects_unbound_identifier() {
    resolve(&mut ResolverCtx::new(), "missing");
}

#[test]
fn resolves_unary_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "-7"),
        ResolvedExpr::Neg(Box::new(ResolvedExpr::Int(7)))
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "-.2.0"),
        ResolvedExpr::FNeg(Box::new(ResolvedExpr::Float(2.0)))
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "!false"),
        ResolvedExpr::Bang(Box::new(ResolvedExpr::Bool(false)))
    );
}

#[test]
fn resolves_integer_arithmetic_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 + 2"),
        ResolvedExpr::Plus(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 - 2"),
        ResolvedExpr::Minus(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 * 2"),
        ResolvedExpr::Mult(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 / 2"),
        ResolvedExpr::Div(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
}

#[test]
fn resolves_float_arithmetic_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 +. 2.0"),
        ResolvedExpr::FPlus(
            Box::new(ResolvedExpr::Float(1.0)),
            Box::new(ResolvedExpr::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 -. 2.0"),
        ResolvedExpr::FMinus(
            Box::new(ResolvedExpr::Float(1.0)),
            Box::new(ResolvedExpr::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 *. 2.0"),
        ResolvedExpr::FMult(
            Box::new(ResolvedExpr::Float(1.0)),
            Box::new(ResolvedExpr::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 /. 2.0"),
        ResolvedExpr::FDiv(
            Box::new(ResolvedExpr::Float(1.0)),
            Box::new(ResolvedExpr::Float(2.0)),
        )
    );
}

#[test]
fn resolves_integer_comparison_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 < 2"),
        ResolvedExpr::Lt(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 <= 2"),
        ResolvedExpr::LtEq(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 > 2"),
        ResolvedExpr::Gt(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 >= 2"),
        ResolvedExpr::GtEq(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
}

#[test]
fn resolves_float_comparison_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 <. 2.0"),
        ResolvedExpr::FLt(
            Box::new(ResolvedExpr::Float(1.0)),
            Box::new(ResolvedExpr::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 <=. 2.0"),
        ResolvedExpr::FLtEq(
            Box::new(ResolvedExpr::Float(1.0)),
            Box::new(ResolvedExpr::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 >. 2.0"),
        ResolvedExpr::FGt(
            Box::new(ResolvedExpr::Float(1.0)),
            Box::new(ResolvedExpr::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 >=. 2.0"),
        ResolvedExpr::FGtEq(
            Box::new(ResolvedExpr::Float(1.0)),
            Box::new(ResolvedExpr::Float(2.0)),
        )
    );
}

#[test]
fn resolves_equality_logical_and_pipe_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 == 2"),
        ResolvedExpr::Eq(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 != 2"),
        ResolvedExpr::NotEq(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true || false"),
        ResolvedExpr::Or(
            Box::new(ResolvedExpr::Bool(true)),
            Box::new(ResolvedExpr::Bool(false)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true && false"),
        ResolvedExpr::And(
            Box::new(ResolvedExpr::Bool(true)),
            Box::new(ResolvedExpr::Bool(false)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 |> 2"),
        ResolvedExpr::Pipe(
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(2)),
        )
    );
}

#[test]
fn resolves_call_expression() {
    let mut ctx = ResolverCtx::new();
    let f = ctx.add_value("f");
    let x = ctx.add_value("x");

    assert_eq!(
        resolve(&mut ctx, "f(9, x, true)"),
        ResolvedExpr::Call(
            Box::new(ResolvedExpr::Id(f)),
            vec![
                ResolvedExpr::Int(9),
                ResolvedExpr::Id(x),
                ResolvedExpr::Bool(true),
            ],
        )
    );
}

#[test]
fn resolves_nested_and_zero_argument_calls() {
    let mut ctx = ResolverCtx::new();
    let f = ctx.add_value("f");
    let g = ctx.add_value("g");

    assert_eq!(
        resolve(&mut ctx, "f()(g)"),
        ResolvedExpr::Call(
            Box::new(ResolvedExpr::Call(Box::new(ResolvedExpr::Id(f)), vec![],)),
            vec![ResolvedExpr::Id(g)],
        )
    );
}

#[test]
fn resolves_if_expression() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "if true then 1 else 0"),
        ResolvedExpr::If(
            Box::new(ResolvedExpr::Bool(true)),
            Box::new(ResolvedExpr::Int(1)),
            Box::new(ResolvedExpr::Int(0)),
        )
    );
}

#[test]
fn resolves_closure_parameter_and_restores_outer_scope() {
    let mut ctx = ResolverCtx::new();
    let outer_x = ctx.add_value("x");
    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_value("x");
    expected_ctx.push_scope();
    let parameter = expected_ctx.add_value("x");
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(&mut ctx, "fn(x: Int) -> x"),
        ResolvedExpr::Fn(
            ResolvedBinding::new(parameter, ResolvedType::Int),
            Box::new(ResolvedExpr::Id(parameter)),
        )
    );
    assert_eq!(ctx.lookup_value("x"), Some(outer_x));
}

#[test]
fn resolves_match_variants_and_arm_bindings() {
    let mut ctx = ResolverCtx::new();
    let scrutinee = ctx.add_value("option");
    let some = ctx.add_value("some");
    let none = ctx.add_value("none");
    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_value("option");
    expected_ctx.add_value("some");
    expected_ctx.add_value("none");
    expected_ctx.push_scope();
    let value = expected_ctx.add_value("value");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(
            &mut ctx,
            "match option { some(value) -> value, none() -> 0 }",
        ),
        ResolvedExpr::Match(
            Box::new(ResolvedExpr::Id(scrutinee)),
            vec![
                ResolvedMatchArm::new(some, vec![value], ResolvedExpr::Id(value)),
                ResolvedMatchArm::new(none, vec![], ResolvedExpr::Int(0)),
            ],
        )
    );
    assert_eq!(ctx.lookup_value("value"), None);
}

#[test]
#[should_panic(expected = "cannot find variant missing")]
fn rejects_match_with_unknown_variant() {
    let mut ctx = ResolverCtx::new();
    ctx.add_value("value");

    resolve(&mut ctx, "match value { missing() -> 0 }");
}

// TODO - this test is invalid right now
// probably install an equivalent once i return Result rather than panicking
// #[test]
// fn panic_while_resolving_closure_does_not_leak_its_scope() {
//     let mut ctx = ResolverCtx::new();
//
//     let result = catch_unwind(AssertUnwindSafe(|| {
//         resolve(&mut ctx, "fn(parameter: Int) -> missing");
//     }));
//
//     assert!(result.is_err());
//     assert_eq!(ctx.lookup_value("parameter"), None);
// }
