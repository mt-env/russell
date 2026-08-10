use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_defn::parse_defn},
    resolution::types::{
        ResolvedBindingData, ResolvedDefnKind, ResolvedExprKind, ResolvedStmtKind, ResolvedType,
        ResolverCtx,
    },
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedDefnKind {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    let parsed = parse_defn(&mut parser).unwrap();
    super::resolve_defn(ctx, parsed)
}

#[test]
fn resolves_simple_function_definition() {
    let mut ctx = ResolverCtx::new();
    let main = ctx.add_value("main");

    assert_eq!(
        resolve(&mut ctx, "fn main() -> Int { return 0; }"),
        ResolvedDefnKind::Fn {
            id: main,
            params: vec![],
            ret_ty: ResolvedType::Int,
            body: vec![ResolvedStmtKind::Return(ResolvedExprKind::Int(0))],
        }
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
        ResolvedDefnKind::Fn {
            id: add,
            params: vec![
                ResolvedBindingData::new(x, ResolvedType::Int),
                ResolvedBindingData::new(y, ResolvedType::Int),
            ],
            ret_ty: ResolvedType::Int,
            body: vec![
                ResolvedStmtKind::Let(
                    sum,
                    ResolvedExprKind::Plus(
                        Box::new(ResolvedExprKind::Id(x)),
                        Box::new(ResolvedExprKind::Id(y)),
                    ),
                ),
                ResolvedStmtKind::Return(ResolvedExprKind::Id(sum)),
            ],
        }
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
        ResolvedDefnKind::Fn {
            id: shadow,
            params: vec![ResolvedBindingData::new(outer_x, ResolvedType::Int)],
            ret_ty: ResolvedType::Int,
            body: vec![
                ResolvedStmtKind::Let(
                    closure,
                    ResolvedExprKind::Fn(
                        ResolvedBindingData::new(inner_x, ResolvedType::Float),
                        Box::new(ResolvedExprKind::Id(inner_x)),
                    ),
                ),
                ResolvedStmtKind::Return(ResolvedExprKind::Id(outer_x)),
            ],
        }
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
    expected_ctx.add_value("value");
    let value = expected_ctx.add_value("value");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    expected_ctx.pop_scope();
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(&mut ctx, "typedef Option('a) { some(value: 'a), none() }"),
        ResolvedDefnKind::Typedef {
            id: option,
            params: vec![parameter],
            arms: vec![
                (
                    some,
                    vec![ResolvedBindingData::new(
                        value,
                        ResolvedType::TypeParam(parameter),
                    )],
                ),
                (none, vec![]),
            ],
        }
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
    expected_ctx.add_value("good");
    let good = expected_ctx.add_value("good");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    expected_ctx.add_value("bad");
    let bad = expected_ctx.add_value("bad");
    expected_ctx.pop_scope();
    expected_ctx.pop_scope();

    assert_eq!(
        resolve(
            &mut ctx,
            "typedef Result('ok, 'err) { ok(good: 'ok), err(bad: 'err) }",
        ),
        ResolvedDefnKind::Typedef {
            id: result,
            params: vec![ok_type, err_type],
            arms: vec![
                (
                    ok,
                    vec![ResolvedBindingData::new(
                        good,
                        ResolvedType::TypeParam(ok_type)
                    )],
                ),
                (
                    err,
                    vec![ResolvedBindingData::new(
                        bad,
                        ResolvedType::TypeParam(err_type)
                    )],
                ),
            ],
        }
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
        ResolvedDefnKind::Fn {
            id: choose,
            params: vec![
                ResolvedBindingData::new(x, ResolvedType::TypeParam(parameter)),
                ResolvedBindingData::new(y, ResolvedType::TypeParam(parameter)),
            ],
            ret_ty: ResolvedType::TypeParam(parameter),
            body: vec![ResolvedStmtKind::Return(ResolvedExprKind::If(
                Box::new(ResolvedExprKind::Bool(true)),
                Box::new(ResolvedExprKind::Id(x)),
                Box::new(ResolvedExprKind::Id(y)),
            ))],
        }
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
        ResolvedDefnKind::Fn {
            id: convert,
            params: vec![
                ResolvedBindingData::new(x, ResolvedType::TypeParam(x_type)),
                ResolvedBindingData::new(y, ResolvedType::TypeParam(y_type)),
            ],
            ret_ty: ResolvedType::TypeParam(return_type),
            body: vec![ResolvedStmtKind::Return(ResolvedExprKind::Plus(
                Box::new(ResolvedExprKind::Id(x)),
                Box::new(ResolvedExprKind::Id(y)),
            ))],
        }
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

    assert!(matches!(
        resolved,
        ResolvedDefnKind::Fn {
            id,
            body,
            ..
        } if id == fib && matches!(
            &body[0],
            ResolvedStmtKind::Return(ResolvedExprKind::If(_, _, else_branch))
                if matches!(
                    &**else_branch,
                    ResolvedExprKind::Call(function, arguments)
                        if **function == ResolvedExprKind::Id(fib)
                            && matches!(&arguments[0], ResolvedExprKind::Minus(left, _)
                                if **left == ResolvedExprKind::Id(n))
                )
        )
    ));
}

#[test]
#[should_panic(expected = "type parameter not found")]
fn rejects_undefined_typedef_type_parameter() {
    let mut ctx = ResolverCtx::new();
    ctx.add_type("ResultErr");
    ctx.add_value("ok");

    resolve(
        &mut ctx,
        "typedef ResultErr('a, 'b) { ok(value: 'missing) }",
    );
}

#[test]
#[should_panic]
fn rejects_duplicate_function_parameter_names() {
    let mut ctx = ResolverCtx::new();
    ctx.add_value("duplicate");

    resolve(
        &mut ctx,
        "fn duplicate(x: Int, x: Float) -> Int { return x + 1; }",
    );
}

#[test]
#[should_panic(expected = "Type Missing not found in scope")]
fn rejects_typedef_missing_from_global_scope() {
    let mut ctx = ResolverCtx::new();
    ctx.add_value("missing");

    resolve(&mut ctx, "typedef Missing { missing() }");
}

#[test]
#[should_panic(expected = "Constructor missing not found in scope")]
fn rejects_constructor_missing_from_global_scope() {
    let mut ctx = ResolverCtx::new();
    ctx.add_type("Missing");

    resolve(&mut ctx, "typedef Missing { missing() }");
}

#[test]
#[should_panic(expected = "Function missing not found in scope")]
fn rejects_function_missing_from_global_scope() {
    resolve(&mut ResolverCtx::new(), "fn missing() -> Int { return 0; }");
}
