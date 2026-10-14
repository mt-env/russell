use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_expr::parse_expr},
    resolution::types::{
        ResolutionErrorKind, ResolvedBinding, ResolvedExpr, ResolvedExprKind, ResolvedMatchArm,
        ResolvedType, ResolverCtx,
    },
    types::Spanned,
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedExpr {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    let parsed = parse_expr(&mut parser).unwrap();
    super::resolve_expr(ctx, parsed)
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
fn resolves_literals() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "42"),
        expr(0, 0, ResolvedExprKind::Int(42))
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "3.5"),
        expr(0, 0, ResolvedExprKind::Float(3.5))
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true"),
        expr(0, 0, ResolvedExprKind::Bool(true))
    );
}

#[test]
fn resolves_identifier_from_context() {
    let mut ctx = ResolverCtx::new();
    let x = ctx.add_value("x");

    assert_eq!(resolve(&mut ctx, "x"), expr(0, 0, ResolvedExprKind::Id(x)));
}

#[test]
fn rejects_unbound_identifier() {
    let mut ctx = ResolverCtx::new();
    resolve(&mut ctx, "missing");
    assert_eq!(
        ctx.errors,
        vec![Spanned {
            offset: 0,
            node: ResolutionErrorKind::UnknownValue { name: "missing" },
        }]
    );
}

#[test]
fn resolves_unary_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "-7"),
        expr(
            0,
            1,
            ResolvedExprKind::Neg(Box::new(expr(1, 0, ResolvedExprKind::Int(7)))),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "-.2.0"),
        expr(
            0,
            1,
            ResolvedExprKind::FNeg(Box::new(expr(2, 0, ResolvedExprKind::Float(2.0)))),
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "!false"),
        expr(
            0,
            1,
            ResolvedExprKind::Bang(Box::new(expr(1, 0, ResolvedExprKind::Bool(false)))),
        )
    );
}

#[test]
fn resolves_integer_arithmetic_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 + 2"),
        expr(
            0,
            2,
            ResolvedExprKind::Plus(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(4, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 - 2"),
        expr(
            0,
            2,
            ResolvedExprKind::Minus(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(4, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 * 2"),
        expr(
            0,
            2,
            ResolvedExprKind::Mult(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(4, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 / 2"),
        expr(
            0,
            2,
            ResolvedExprKind::Div(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(4, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
}

#[test]
fn resolves_float_arithmetic_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 +. 2.0"),
        expr(
            0,
            2,
            ResolvedExprKind::FPlus(
                Box::new(expr(0, 0, ResolvedExprKind::Float(1.0))),
                Box::new(expr(7, 1, ResolvedExprKind::Float(2.0))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 -. 2.0"),
        expr(
            0,
            2,
            ResolvedExprKind::FMinus(
                Box::new(expr(0, 0, ResolvedExprKind::Float(1.0))),
                Box::new(expr(7, 1, ResolvedExprKind::Float(2.0))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 *. 2.0"),
        expr(
            0,
            2,
            ResolvedExprKind::FMult(
                Box::new(expr(0, 0, ResolvedExprKind::Float(1.0))),
                Box::new(expr(7, 1, ResolvedExprKind::Float(2.0))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 /. 2.0"),
        expr(
            0,
            2,
            ResolvedExprKind::FDiv(
                Box::new(expr(0, 0, ResolvedExprKind::Float(1.0))),
                Box::new(expr(7, 1, ResolvedExprKind::Float(2.0))),
            )
        )
    );
}

#[test]
fn resolves_integer_comparison_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 < 2"),
        expr(
            0,
            2,
            ResolvedExprKind::Lt(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(4, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 <= 2"),
        expr(
            0,
            2,
            ResolvedExprKind::LtEq(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(5, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 > 2"),
        expr(
            0,
            2,
            ResolvedExprKind::Gt(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(4, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 >= 2"),
        expr(
            0,
            2,
            ResolvedExprKind::GtEq(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(5, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
}

#[test]
fn resolves_float_comparison_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 <. 2.0"),
        expr(
            0,
            2,
            ResolvedExprKind::FLt(
                Box::new(expr(0, 0, ResolvedExprKind::Float(1.0))),
                Box::new(expr(7, 1, ResolvedExprKind::Float(2.0))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 <=. 2.0"),
        expr(
            0,
            2,
            ResolvedExprKind::FLtEq(
                Box::new(expr(0, 0, ResolvedExprKind::Float(1.0))),
                Box::new(expr(8, 1, ResolvedExprKind::Float(2.0))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 >. 2.0"),
        expr(
            0,
            2,
            ResolvedExprKind::FGt(
                Box::new(expr(0, 0, ResolvedExprKind::Float(1.0))),
                Box::new(expr(7, 1, ResolvedExprKind::Float(2.0))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1.0 >=. 2.0"),
        expr(
            0,
            2,
            ResolvedExprKind::FGtEq(
                Box::new(expr(0, 0, ResolvedExprKind::Float(1.0))),
                Box::new(expr(8, 1, ResolvedExprKind::Float(2.0))),
            )
        )
    );
}

#[test]
fn resolves_equality_logical_and_pipe_expressions() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 == 2"),
        expr(
            0,
            2,
            ResolvedExprKind::Eq(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(5, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 != 2"),
        expr(
            0,
            2,
            ResolvedExprKind::NotEq(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(5, 1, ResolvedExprKind::Int(2))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true || false"),
        expr(
            0,
            2,
            ResolvedExprKind::Or(
                Box::new(expr(0, 0, ResolvedExprKind::Bool(true))),
                Box::new(expr(8, 1, ResolvedExprKind::Bool(false))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "true && false"),
        expr(
            0,
            2,
            ResolvedExprKind::And(
                Box::new(expr(0, 0, ResolvedExprKind::Bool(true))),
                Box::new(expr(8, 1, ResolvedExprKind::Bool(false))),
            )
        )
    );
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "1 |> 2"),
        expr(
            0,
            2,
            ResolvedExprKind::Pipe(
                Box::new(expr(0, 0, ResolvedExprKind::Int(1))),
                Box::new(expr(5, 1, ResolvedExprKind::Int(2))),
            )
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
        expr(
            0,
            4,
            ResolvedExprKind::Call(
                Box::new(expr(0, 0, ResolvedExprKind::Id(f))),
                vec![
                    expr(2, 1, ResolvedExprKind::Int(9)),
                    expr(5, 2, ResolvedExprKind::Id(x)),
                    expr(8, 3, ResolvedExprKind::Bool(true)),
                ],
            )
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
        expr(
            0,
            3,
            ResolvedExprKind::Call(
                Box::new(expr(
                    0,
                    1,
                    ResolvedExprKind::Call(Box::new(expr(0, 0, ResolvedExprKind::Id(f))), vec![],)
                )),
                vec![expr(4, 2, ResolvedExprKind::Id(g))],
            )
        )
    );
}

#[test]
fn resolves_if_expression() {
    assert_eq!(
        resolve(&mut ResolverCtx::new(), "if true then 1 else 0"),
        expr(
            0,
            3,
            ResolvedExprKind::If(
                Box::new(expr(3, 0, ResolvedExprKind::Bool(true))),
                Box::new(expr(13, 1, ResolvedExprKind::Int(1))),
                Box::new(expr(20, 2, ResolvedExprKind::Int(0))),
            )
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
        expr(
            0,
            1,
            ResolvedExprKind::Fn(
                ResolvedBinding::new(3, parameter, ResolvedType::mk_int(6)),
                Box::new(expr(14, 0, ResolvedExprKind::Id(parameter))),
            )
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
        expr(
            0,
            3,
            ResolvedExprKind::Match(
                Box::new(expr(6, 0, ResolvedExprKind::Id(scrutinee))),
                vec![
                    ResolvedMatchArm::new(
                        15,
                        some,
                        vec![value],
                        expr(30, 1, ResolvedExprKind::Id(value)),
                    ),
                    ResolvedMatchArm::new(37, none, vec![], expr(47, 2, ResolvedExprKind::Int(0)),),
                ],
            )
        )
    );
    assert_eq!(ctx.lookup_value("value"), None);
}

#[test]
fn rejects_match_with_unknown_variant() {
    let mut ctx = ResolverCtx::new();
    ctx.add_value("value");

    resolve(&mut ctx, "match value { missing() -> 0 }");
    assert_eq!(
        ctx.errors,
        vec![Spanned {
            offset: 14,
            node: ResolutionErrorKind::UnknownValue { name: "missing" },
        }]
    );
}

#[test]
fn rejects_duplicate_match_binding_names() {
    let mut ctx = ResolverCtx::new();
    ctx.add_value("option");
    ctx.add_value("some");

    resolve(&mut ctx, "match option { some(x, x) -> x }");

    assert_eq!(
        ctx.errors,
        vec![Spanned {
            offset: 15,
            node: ResolutionErrorKind::DuplicateValue { name: "x" },
        }]
    );
}
