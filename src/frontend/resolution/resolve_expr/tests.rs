use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_expr::parse_expr},
    resolution::types::{
        ResolvedBindingData, ResolvedExprKind, ResolvedMatchArm, ResolvedType, ResolverCtx,
    },
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedExprKind {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    let parsed = parse_expr(&mut parser).unwrap();
    super::resolve_expr(ctx, parsed)
}

#[test]
fn resolves_literals() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "42"),
        ResolvedExprKind::Int(42)
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "3.5"),
        ResolvedExprKind::Float(3.5)
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true"),
        ResolvedExprKind::Bool(true)
    );
}

#[test]
fn resolves_identifier_from_context() {
    let mut ctx = ResolverCtx::new();
    let x = ctx.add_value("x");

    assert_eq!(resolve(&mut ctx, "x"), ResolvedExprKind::Id(x));
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
        ResolvedExprKind::Neg(Box::new(ResolvedExprKind::Int(7)))
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "-.2.0"),
        ResolvedExprKind::FNeg(Box::new(ResolvedExprKind::Float(2.0)))
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "!false"),
        ResolvedExprKind::Bang(Box::new(ResolvedExprKind::Bool(false)))
    );
}

#[test]
fn resolves_integer_arithmetic_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 + 2"),
        ResolvedExprKind::Plus(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 - 2"),
        ResolvedExprKind::Minus(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 * 2"),
        ResolvedExprKind::Mult(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 / 2"),
        ResolvedExprKind::Div(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
}

#[test]
fn resolves_float_arithmetic_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 +. 2.0"),
        ResolvedExprKind::FPlus(
            Box::new(ResolvedExprKind::Float(1.0)),
            Box::new(ResolvedExprKind::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 -. 2.0"),
        ResolvedExprKind::FMinus(
            Box::new(ResolvedExprKind::Float(1.0)),
            Box::new(ResolvedExprKind::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 *. 2.0"),
        ResolvedExprKind::FMult(
            Box::new(ResolvedExprKind::Float(1.0)),
            Box::new(ResolvedExprKind::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 /. 2.0"),
        ResolvedExprKind::FDiv(
            Box::new(ResolvedExprKind::Float(1.0)),
            Box::new(ResolvedExprKind::Float(2.0)),
        )
    );
}

#[test]
fn resolves_integer_comparison_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 < 2"),
        ResolvedExprKind::Lt(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 <= 2"),
        ResolvedExprKind::LtEq(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 > 2"),
        ResolvedExprKind::Gt(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 >= 2"),
        ResolvedExprKind::GtEq(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
}

#[test]
fn resolves_float_comparison_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 <. 2.0"),
        ResolvedExprKind::FLt(
            Box::new(ResolvedExprKind::Float(1.0)),
            Box::new(ResolvedExprKind::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 <=. 2.0"),
        ResolvedExprKind::FLtEq(
            Box::new(ResolvedExprKind::Float(1.0)),
            Box::new(ResolvedExprKind::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 >. 2.0"),
        ResolvedExprKind::FGt(
            Box::new(ResolvedExprKind::Float(1.0)),
            Box::new(ResolvedExprKind::Float(2.0)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 >=. 2.0"),
        ResolvedExprKind::FGtEq(
            Box::new(ResolvedExprKind::Float(1.0)),
            Box::new(ResolvedExprKind::Float(2.0)),
        )
    );
}

#[test]
fn resolves_equality_logical_and_pipe_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 == 2"),
        ResolvedExprKind::Eq(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 != 2"),
        ResolvedExprKind::NotEq(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true || false"),
        ResolvedExprKind::Or(
            Box::new(ResolvedExprKind::Bool(true)),
            Box::new(ResolvedExprKind::Bool(false)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true && false"),
        ResolvedExprKind::And(
            Box::new(ResolvedExprKind::Bool(true)),
            Box::new(ResolvedExprKind::Bool(false)),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 |> 2"),
        ResolvedExprKind::Pipe(
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(2)),
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
        ResolvedExprKind::Call(
            Box::new(ResolvedExprKind::Id(f)),
            vec![
                ResolvedExprKind::Int(9),
                ResolvedExprKind::Id(x),
                ResolvedExprKind::Bool(true),
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
        ResolvedExprKind::Call(
            Box::new(ResolvedExprKind::Call(
                Box::new(ResolvedExprKind::Id(f)),
                vec![],
            )),
            vec![ResolvedExprKind::Id(g)],
        )
    );
}

#[test]
fn resolves_if_expression() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "if true then 1 else 0"),
        ResolvedExprKind::If(
            Box::new(ResolvedExprKind::Bool(true)),
            Box::new(ResolvedExprKind::Int(1)),
            Box::new(ResolvedExprKind::Int(0)),
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
        ResolvedExprKind::Fn(
            ResolvedBindingData::new(parameter, ResolvedType::Int),
            Box::new(ResolvedExprKind::Id(parameter)),
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
        ResolvedExprKind::Match(
            Box::new(ResolvedExprKind::Id(scrutinee)),
            vec![
                ResolvedMatchArm::new(some, vec![value], ResolvedExprKind::Id(value)),
                ResolvedMatchArm::new(none, vec![], ResolvedExprKind::Int(0)),
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
