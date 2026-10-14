use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_defn::parse_defn},
    resolution::types::{
        ResolutionErrorKind, ResolvedBinding, ResolvedConstructorArm, ResolvedDefn, ResolvedExpr,
        ResolvedExprKind, ResolvedStmt, ResolvedType, ResolverCtx,
    },
    types::Spanned,
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedDefn {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    let parsed = parse_defn(&mut parser).unwrap();
    super::resolve_defn(ctx, parsed)
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
fn resolves_simple_function_definition() {
    let mut ctx = ResolverCtx::new();
    let main = ctx.add_value("main");

    assert_eq!(
        resolve(&mut ctx, "fn main() -> Int { return 0; }"),
        ResolvedDefn::mk_fn(
            0,
            main,
            vec![],
            ResolvedType::mk_int(13),
            vec![ResolvedStmt::mk_return(
                19,
                expr(26, 0, ResolvedExprKind::Int(0)),
            )],
        )
    );
}

#[test]
fn resolves_function_parameters_and_body_bindings() {
    let mut ctx = ResolverCtx::new();
    let add = ctx.add_value("add");
    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_value("add");
    expected_ctx.push_scope();
    let x = expected_ctx.add_value("x");
    let y = expected_ctx.add_value("y");
    let sum = expected_ctx.add_value("sum");
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(
            &mut ctx,
            "fn add(x: Int, y: Int) -> Int { let sum = x + y; return sum; }",
        ),
        ResolvedDefn::mk_fn(
            0,
            add,
            vec![
                ResolvedBinding::new(7, x, ResolvedType::mk_int(10)),
                ResolvedBinding::new(15, y, ResolvedType::mk_int(18)),
            ],
            ResolvedType::mk_int(26),
            vec![
                ResolvedStmt::mk_let(
                    32,
                    sum,
                    expr(
                        42,
                        2,
                        ResolvedExprKind::Plus(
                            Box::new(expr(42, 0, ResolvedExprKind::Id(x))),
                            Box::new(expr(46, 1, ResolvedExprKind::Id(y))),
                        ),
                    ),
                ),
                ResolvedStmt::mk_return(49, expr(56, 3, ResolvedExprKind::Id(sum))),
            ],
        )
    );
    assert_eq!(ctx.lookup_value("x"), None);
    assert_eq!(ctx.lookup_value("sum"), None);
}

#[test]
fn closure_parameter_shadows_function_parameter_only_inside_closure() {
    let mut ctx = ResolverCtx::new();
    let shadow = ctx.add_value("shadow");

    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_value("shadow");
    expected_ctx.push_scope();
    let outer_x = expected_ctx.add_value("x");
    expected_ctx.push_scope();
    let inner_x = expected_ctx.add_value("x");
    expected_ctx.pop_scope();
    let closure = expected_ctx.add_value("closure");
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(
            &mut ctx,
            "fn shadow(x: Int) -> Int { \
                let closure = fn(x: Float) -> x; \
                return x; \
            }",
        ),
        ResolvedDefn::mk_fn(
            0,
            shadow,
            vec![ResolvedBinding::new(10, outer_x, ResolvedType::mk_int(13))],
            ResolvedType::mk_int(21),
            vec![
                ResolvedStmt::mk_let(
                    27,
                    closure,
                    expr(
                        41,
                        1,
                        ResolvedExprKind::Fn(
                            ResolvedBinding::new(44, inner_x, ResolvedType::mk_float(47)),
                            Box::new(expr(57, 0, ResolvedExprKind::Id(inner_x))),
                        ),
                    ),
                ),
                ResolvedStmt::mk_return(60, expr(67, 2, ResolvedExprKind::Id(outer_x))),
            ],
        )
    );
}

#[test]
fn resolves_typedef_definition() {
    let mut ctx = ResolverCtx::new();
    let option = ctx.add_type("Option");
    let some = ctx.add_value("some");
    let none = ctx.add_value("none");

    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_type("Option");
    expected_ctx.add_value("some");
    expected_ctx.add_value("none");
    expected_ctx.push_scope();
    let parameter = expected_ctx.add_typeparam("'a");
    expected_ctx.push_scope();
    let value = expected_ctx.add_value("value");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    expected_ctx.pop_scope();
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(&mut ctx, "typedef Option('a) { some(value: 'a), none() }"),
        ResolvedDefn::mk_typedef(
            0,
            option,
            vec![parameter],
            vec![
                ResolvedConstructorArm::new(
                    21,
                    some,
                    vec![ResolvedBinding::new(
                        26,
                        value,
                        ResolvedType::mk_typeparam(33, parameter),
                    )],
                ),
                ResolvedConstructorArm::new(38, none, vec![]),
            ],
        )
    );
}

#[test]
fn resolves_typedef_with_multiple_parameters_and_arms() {
    let mut ctx = ResolverCtx::new();
    let result = ctx.add_type("Result");
    let ok = ctx.add_value("ok");
    let err = ctx.add_value("err");

    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_type("Result");
    expected_ctx.add_value("ok");
    expected_ctx.add_value("err");
    expected_ctx.push_scope();
    let ok_type = expected_ctx.add_typeparam("'ok");
    let err_type = expected_ctx.add_typeparam("'err");
    expected_ctx.push_scope();
    let good = expected_ctx.add_value("good");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    let bad = expected_ctx.add_value("bad");
    expected_ctx.pop_scope();
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(
            &mut ctx,
            "typedef Result('ok, 'err) { ok(good: 'ok), err(bad: 'err) }",
        ),
        ResolvedDefn::mk_typedef(
            0,
            result,
            vec![ok_type, err_type],
            vec![
                ResolvedConstructorArm::new(
                    28,
                    ok,
                    vec![ResolvedBinding::new(
                        31,
                        good,
                        ResolvedType::mk_typeparam(37, ok_type)
                    )],
                ),
                ResolvedConstructorArm::new(
                    43,
                    err,
                    vec![ResolvedBinding::new(
                        47,
                        bad,
                        ResolvedType::mk_typeparam(52, err_type)
                    )],
                ),
            ],
        )
    );
}

#[test]
fn repeated_generic_function_type_parameter_resolves_to_one_id() {
    let mut ctx = ResolverCtx::new();
    let choose = ctx.add_value("choose");
    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_value("choose");
    expected_ctx.push_scope();
    let parameter = expected_ctx.add_typeparam("'a");
    let x = expected_ctx.add_value("x");
    let y = expected_ctx.add_value("y");
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(
            &mut ctx,
            "fn choose(x: 'a, y: 'a) -> 'a { return if true then x else y; }",
        ),
        ResolvedDefn::mk_fn(
            0,
            choose,
            vec![
                ResolvedBinding::new(10, x, ResolvedType::mk_typeparam(13, parameter)),
                ResolvedBinding::new(17, y, ResolvedType::mk_typeparam(20, parameter)),
            ],
            ResolvedType::mk_typeparam(27, parameter),
            vec![ResolvedStmt::mk_return(
                32,
                expr(
                    39,
                    3,
                    ResolvedExprKind::If(
                        Box::new(expr(42, 0, ResolvedExprKind::Bool(true))),
                        Box::new(expr(52, 1, ResolvedExprKind::Id(x))),
                        Box::new(expr(59, 2, ResolvedExprKind::Id(y))),
                    ),
                ),
            )],
        )
    );
}

#[test]
fn distinct_generic_function_type_parameters_resolve_to_distinct_ids() {
    let mut ctx = ResolverCtx::new();
    let convert = ctx.add_value("convert");
    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_value("convert");
    expected_ctx.push_scope();
    let return_type = expected_ctx.add_typeparam("'c");
    let x = expected_ctx.add_value("x");
    let x_type = expected_ctx.add_typeparam("'a");
    let y = expected_ctx.add_value("y");
    let y_type = expected_ctx.add_typeparam("'b");
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(&mut ctx, "fn convert(x: 'a, y: 'b) -> 'c { return x + y; }"),
        ResolvedDefn::mk_fn(
            0,
            convert,
            vec![
                ResolvedBinding::new(11, x, ResolvedType::mk_typeparam(14, x_type)),
                ResolvedBinding::new(18, y, ResolvedType::mk_typeparam(21, y_type)),
            ],
            ResolvedType::mk_typeparam(28, return_type),
            vec![ResolvedStmt::mk_return(
                33,
                expr(
                    40,
                    2,
                    ResolvedExprKind::Plus(
                        Box::new(expr(40, 0, ResolvedExprKind::Id(x))),
                        Box::new(expr(44, 1, ResolvedExprKind::Id(y))),
                    ),
                ),
            )],
        )
    );
}

#[test]
fn resolves_recursive_function_reference_to_global_id() {
    let mut ctx = ResolverCtx::new();
    let fib = ctx.add_value("fib");
    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_value("fib");
    expected_ctx.push_scope();
    let n = expected_ctx.add_value("n");
    expected_ctx.pop_scope();

    let resolved = resolve(
        &mut ctx,
        "fn fib(n: Int) -> Int { return if n <= 1 then n else fib(n - 1); }",
    );

    assert_eq!(
        resolved,
        ResolvedDefn::mk_fn(
            0,
            fib,
            vec![ResolvedBinding::new(7, n, ResolvedType::mk_int(10))],
            ResolvedType::mk_int(18),
            vec![ResolvedStmt::mk_return(
                24,
                expr(
                    31,
                    9,
                    ResolvedExprKind::If(
                        Box::new(expr(
                            34,
                            2,
                            ResolvedExprKind::LtEq(
                                Box::new(expr(34, 0, ResolvedExprKind::Id(n))),
                                Box::new(expr(39, 1, ResolvedExprKind::Int(1))),
                            ),
                        )),
                        Box::new(expr(46, 3, ResolvedExprKind::Id(n))),
                        Box::new(expr(
                            53,
                            8,
                            ResolvedExprKind::Call(
                                Box::new(expr(53, 4, ResolvedExprKind::Id(fib))),
                                vec![expr(
                                    57,
                                    7,
                                    ResolvedExprKind::Minus(
                                        Box::new(expr(57, 5, ResolvedExprKind::Id(n))),
                                        Box::new(expr(61, 6, ResolvedExprKind::Int(1))),
                                    ),
                                )],
                            ),
                        )),
                    ),
                ),
            )],
        )
    );
}

#[test]
fn rejects_undefined_typedef_type_parameter() {
    let mut ctx = ResolverCtx::new();
    ctx.add_type("ResultErr");
    ctx.add_value("ok");

    resolve(
        &mut ctx,
        "typedef ResultErr('a, 'b) { ok(value: 'missing) }",
    );
    assert_eq!(
        ctx.errors.as_slice(),
        &[Spanned {
            offset: 38,
            node: ResolutionErrorKind::UnknownType { name: "'missing" },
        }]
    );
}

#[test]
fn rejects_undefined_typedef_type() {
    let mut ctx = ResolverCtx::new();
    ctx.add_type("Box");
    ctx.add_value("wrap");

    resolve(&mut ctx, "typedef Box { wrap(value: Missing) }");

    assert_eq!(
        ctx.errors,
        vec![Spanned {
            offset: 26,
            node: ResolutionErrorKind::UnknownType { name: "Missing" },
        }]
    );
}

#[test]
fn eejects_duplicate_typedef_binding_names() {
    let mut ctx = ResolverCtx::new();
    ctx.add_type("Pair");
    ctx.add_value("pair");

    resolve(&mut ctx, "typedef Pair { pair(x: Int, x: Bool) }");

    assert_eq!(
        ctx.errors,
        vec![Spanned {
            offset: 28,
            node: ResolutionErrorKind::DuplicateValue { name: "x" },
        }]
    );
}

#[test]
fn rejects_duplicate_function_parameter_names() {
    let mut ctx = ResolverCtx::new();
    ctx.add_value("duplicate");

    resolve(
        &mut ctx,
        "fn duplicate(x: Int, x: Float) -> Int { return x + 1; }",
    );
    assert_eq!(
        ctx.errors,
        vec![Spanned {
            offset: 21,
            node: ResolutionErrorKind::DuplicateValue { name: "x" },
        }]
    );
}

#[test]
#[should_panic(expected = "COMPILER BUG: cannot find type Missing")]
fn rejects_typedef_missing_from_global_scope() {
    let mut ctx = ResolverCtx::new();
    ctx.add_value("missing");

    resolve(&mut ctx, "typedef Missing { missing() }");
}

#[test]
#[should_panic(expected = "COMPILER BUG: cannot find constructor missing")]
fn rejects_constructor_missing_from_global_scope() {
    let mut ctx = ResolverCtx::new();
    ctx.add_type("Missing");

    resolve(&mut ctx, "typedef Missing { missing() }");
}

#[test]
#[should_panic(expected = "COMPILER BUG: cannot find function missing")]
fn rejects_function_missing_from_global_scope() {
    resolve(&mut ResolverCtx::new(), "fn missing() -> Int { return 0; }");
}
