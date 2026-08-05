use crate::frontend::{
    lexer::lex,
    parser::parse,
    resolution::types::{
        ResolvedBinding, ResolvedDefn, ResolvedExpr, ResolvedStmt, ResolvedType, ResolverCtx,
    },
};

fn resolve(input: &str) -> Vec<ResolvedDefn> {
    super::resolve(parse(lex(input)))
}

#[test]
fn resolves_single_function_program() {
    assert!(matches!(
        resolve("fn main() -> Int { return 0; }").as_slice(),
        [ResolvedDefn::Fn {
            params,
            ret_ty: ResolvedType::Int,
            body,
            ..
        }] if params.is_empty()
            && body == &[ResolvedStmt::Return(ResolvedExpr::Int(0))]
    ));
}

#[test]
fn resolves_typedef_and_function_program() {
    let resolved = resolve(
        "typedef Option('a) { some(value: 'a), none() } \
         fn main() -> Int { return 0; }",
    );

    assert!(matches!(
        resolved.as_slice(),
        [
            ResolvedDefn::Typedef { params, arms, .. },
            ResolvedDefn::Fn {
                ret_ty: ResolvedType::Int,
                body,
                ..
            },
        ] if params.len() == 1
            && arms.len() == 2
            && arms[0].1.len() == 1
            && arms[1].1.is_empty()
            && body == &[ResolvedStmt::Return(ResolvedExpr::Int(0))]
    ));
}

#[test]
fn resolves_reference_to_later_global_function() {
    let resolved = resolve(
        "fn main() -> Int { return later(41); } \
         fn later(x: Int) -> Int { return x + 1; }",
    );

    let [
        ResolvedDefn::Fn {
            body: main_body, ..
        },
        ResolvedDefn::Fn { id: later, .. },
    ] = resolved.as_slice()
    else {
        panic!("expected two function definitions");
    };

    assert!(matches!(
        &main_body[0],
        ResolvedStmt::Return(ResolvedExpr::Call(function, arguments))
            if **function == ResolvedExpr::Id(*later)
                && arguments == &[ResolvedExpr::Int(41)]
    ));
}

#[test]
fn resolves_mutually_recursive_functions() {
    let resolved = resolve(
        "fn even(x: Int) -> Bool { return if x == 0 then true else odd(x - 1); } \
         fn odd(x: Int) -> Bool { return if x == 0 then false else even(x - 1); } \
         fn main() -> Int { return 0; }",
    );

    let [
        ResolvedDefn::Fn {
            id: even,
            body: even_body,
            ..
        },
        ResolvedDefn::Fn {
            id: odd,
            body: odd_body,
            ..
        },
        ResolvedDefn::Fn { .. },
    ] = resolved.as_slice()
    else {
        panic!("expected three function definitions");
    };

    assert!(branch_calls(&even_body[0], *odd));
    assert!(branch_calls(&odd_body[0], *even));
}

fn branch_calls(stmt: &ResolvedStmt, expected: super::types::ValueId) -> bool {
    matches!(
        stmt,
        ResolvedStmt::Return(ResolvedExpr::If(_, _, branch))
            if matches!(
                &**branch,
                ResolvedExpr::Call(function, _)
                    if **function == ResolvedExpr::Id(expected)
            )
    )
}

#[test]
fn resolves_mutually_recursive_types() {
    let resolved = resolve(
        "typedef RecursiveA('a) { a(value: 'a), b(next: RecursiveB('a)) } \
         typedef RecursiveB('b) { c(value: 'b), d(next: RecursiveA('b)) } \
         fn main() -> Int { return 0; }",
    );

    let mut expected_ctx = ResolverCtx::new();
    let recursive_a = expected_ctx.add_type("RecursiveA");
    let a = expected_ctx.add_value("a");
    let b = expected_ctx.add_value("b");
    let recursive_b = expected_ctx.add_type("RecursiveB");
    let c = expected_ctx.add_value("c");
    let d = expected_ctx.add_value("d");
    let main = expected_ctx.add_value("main");

    expected_ctx.push_scope();
    let a_type = expected_ctx.add_typeparam("'a");
    expected_ctx.push_scope();
    expected_ctx.add_value("value");
    let a_value = expected_ctx.add_value("value");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    expected_ctx.add_value("next");
    let b_next = expected_ctx.add_value("next");
    expected_ctx.pop_scope();
    expected_ctx.pop_scope();

    expected_ctx.push_scope();
    let b_type = expected_ctx.add_typeparam("'b");
    expected_ctx.push_scope();
    expected_ctx.add_value("value");
    let b_value = expected_ctx.add_value("value");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    expected_ctx.add_value("next");
    let a_next = expected_ctx.add_value("next");
    expected_ctx.pop_scope();
    expected_ctx.pop_scope();

    expected_ctx.push_scope();
    expected_ctx.pop_scope();

    assert_eq!(
        resolved,
        vec![
            ResolvedDefn::Typedef {
                id: recursive_a,
                params: vec![a_type],
                arms: vec![
                    (
                        a,
                        vec![ResolvedBinding::new(
                            a_value,
                            ResolvedType::TypeParam(a_type),
                        )],
                    ),
                    (
                        b,
                        vec![ResolvedBinding::new(
                            b_next,
                            ResolvedType::TypeApp(
                                recursive_b,
                                vec![ResolvedType::TypeParam(a_type)],
                            ),
                        )],
                    ),
                ],
            },
            ResolvedDefn::Typedef {
                id: recursive_b,
                params: vec![b_type],
                arms: vec![
                    (
                        c,
                        vec![ResolvedBinding::new(
                            b_value,
                            ResolvedType::TypeParam(b_type),
                        )],
                    ),
                    (
                        d,
                        vec![ResolvedBinding::new(
                            a_next,
                            ResolvedType::TypeApp(
                                recursive_a,
                                vec![ResolvedType::TypeParam(b_type)],
                            ),
                        )],
                    ),
                ],
            },
            ResolvedDefn::Fn {
                id: main,
                params: vec![],
                ret_ty: ResolvedType::Int,
                body: vec![ResolvedStmt::Return(ResolvedExpr::Int(0))],
            },
        ]
    );
}

#[test]
#[should_panic]
fn rejects_empty_program() {
    resolve("");
}

#[test]
#[should_panic]
fn rejects_program_without_main_function() {
    resolve("fn helper() -> Int { return 0; }");
}

#[test]
#[should_panic]
fn rejects_typedef_only_program() {
    resolve("typedef Unit { unit() }");
}

#[test]
#[should_panic]
fn rejects_duplicate_function_name() {
    resolve(
        "fn duplicate() -> Int { return 0; } \
         fn duplicate() -> Int { return 1; } \
         fn main() -> Int { return 0; }",
    );
}

#[test]
#[should_panic]
fn rejects_duplicate_function_name_with_different_parameters() {
    resolve(
        "fn duplicate() -> Int { return 0; } \
         fn duplicate(x: Int) -> Int { return x; } \
         fn main() -> Int { return 0; }",
    );
}

#[test]
#[should_panic]
fn rejects_function_and_constructor_name_collision() {
    resolve(
        "fn duplicate() -> Int { return 0; } \
         typedef Duplicate { duplicate() } \
         fn main() -> Int { return 0; }",
    );
}

#[test]
#[should_panic]
fn rejects_function_and_constructor_name_collision_with_different_parameters() {
    resolve(
        "fn duplicate() -> Int { return 0; } \
         typedef Duplicate { duplicate(value: Int) } \
         fn main() -> Int { return 0; }",
    );
}

#[test]
#[should_panic]
fn rejects_duplicate_typedef_name() {
    resolve(
        "typedef Duplicate { first() } \
         typedef Duplicate { second() } \
         fn main() -> Int { return 0; }",
    );
}

#[test]
#[should_panic]
fn rejects_duplicate_typedef_name_with_different_type_parameters() {
    resolve(
        "typedef Duplicate { first() } \
         typedef Duplicate('a) { second(value: 'a) } \
         fn main() -> Int { return 0; }",
    );
}

#[test]
#[should_panic]
fn rejects_nullary_constructor_name_collision_across_typedefs() {
    resolve(
        "typedef First { duplicate() } \
         typedef Second { duplicate() } \
         fn main() -> Int { return 0; }",
    );
}

#[test]
#[should_panic]
fn rejects_constructor_name_collision_across_typedefs() {
    resolve(
        "typedef First { duplicate() } \
         typedef Second { duplicate(value: Int) } \
         fn main() -> Int { return 0; }",
    );
}

#[test]
fn same_generic_name_in_separate_functions_gets_distinct_ids() {
    let resolved = resolve(
        "fn first(x: 'a) -> 'a { return x; } \
         fn second(x: 'a) -> 'a { return x; } \
         fn main() -> Int { return 0; }",
    );

    let [
        ResolvedDefn::Fn {
            ret_ty: first_type, ..
        },
        ResolvedDefn::Fn {
            ret_ty: second_type,
            ..
        },
        ResolvedDefn::Fn { .. },
    ] = resolved.as_slice()
    else {
        panic!("expected three function definitions");
    };

    assert_ne!(first_type, second_type);
}
