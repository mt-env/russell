use crate::frontend::{
    lexer::lex,
    parser::parse,
    resolution::types::{
        ResolutionError, ResolutionErrorKind, ResolvedBinding, ResolvedConstructorArm,
        ResolvedDefn, ResolvedExpr, ResolvedExprKind, ResolvedStmt, ResolvedType, ResolverCtx,
    },
    types::Spanned,
};

fn resolve(input: &str) -> Vec<ResolvedDefn> {
    let result = super::resolve(parse(
        lex(input).expect("test input should lex successfully"),
    ));
    match result {
        Ok(success) => success.resolved_defns,
        Err(_) => panic!("test input should resolve successfully"),
    }
}

fn assert_resolution_error(input: &str, expected: ResolutionError<'_>) {
    let defns = parse(lex(input).expect("test input should lex successfully"));
    let mut ctx = ResolverCtx::new();
    super::init_global_scope(&mut ctx, &defns);
    for defn in defns {
        super::resolve_defn::resolve_defn(&mut ctx, defn);
    }
    assert_eq!(ctx.errors, vec![expected]);
}

fn resolve_result<'a>(
    input: &'a str,
) -> Result<super::types::ResolutionSuccess<'a>, super::types::ResolutionFailure<'a>> {
    super::resolve(parse(
        lex(input).expect("test input should lex successfully"),
    ))
}

fn errors<'a>(input: &'a str) -> Vec<ResolutionError<'a>> {
    match resolve_result(input) {
        Ok(_) => panic!("expected resolution to fail"),
        Err(failure) => failure.errors,
    }
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
fn resolves_single_function_program() {
    let mut expected_ctx = ResolverCtx::new();
    let main = expected_ctx.add_value("main");

    assert_eq!(
        resolve("fn main() -> Int { return 0; }"),
        vec![ResolvedDefn::mk_fn(
            0,
            main,
            vec![],
            ResolvedType::mk_int(13),
            vec![ResolvedStmt::mk_return(
                19,
                expr(26, 0, ResolvedExprKind::Int(0)),
            )],
        )]
    );
}

#[test]
fn resolves_typedef_and_function_program() {
    let mut expected_ctx = ResolverCtx::new();
    let option = expected_ctx.add_type("Option");
    let some = expected_ctx.add_value("some");
    let none = expected_ctx.add_value("none");
    let main = expected_ctx.add_value("main");
    let parameter = expected_ctx.add_typeparam("'a");
    let value = expected_ctx.add_value("value");

    assert_eq!(
        resolve(
            "typedef Option('a) { some(value: 'a), none() } \
         fn main() -> Int { return 0; }",
        ),
        vec![
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
            ),
            ResolvedDefn::mk_fn(
                47,
                main,
                vec![],
                ResolvedType::mk_int(60),
                vec![ResolvedStmt::mk_return(
                    66,
                    expr(73, 0, ResolvedExprKind::Int(0)),
                )],
            ),
        ]
    );
}

#[test]
fn resolves_reference_to_later_global_function() {
    let mut expected_ctx = ResolverCtx::new();
    let main = expected_ctx.add_value("main");
    let later = expected_ctx.add_value("later");
    let x = expected_ctx.add_value("x");

    assert_eq!(
        resolve(
            "fn main() -> Int { return later(41); } \
             fn later(x: Int) -> Int { return x + 1; }",
        ),
        vec![
            ResolvedDefn::mk_fn(
                0,
                main,
                vec![],
                ResolvedType::mk_int(13),
                vec![ResolvedStmt::mk_return(
                    19,
                    expr(
                        26,
                        2,
                        ResolvedExprKind::Call(
                            Box::new(expr(26, 0, ResolvedExprKind::Id(later))),
                            vec![expr(32, 1, ResolvedExprKind::Int(41))],
                        ),
                    ),
                )],
            ),
            ResolvedDefn::mk_fn(
                39,
                later,
                vec![ResolvedBinding::new(48, x, ResolvedType::mk_int(51))],
                ResolvedType::mk_int(59),
                vec![ResolvedStmt::mk_return(
                    65,
                    expr(
                        72,
                        5,
                        ResolvedExprKind::Plus(
                            Box::new(expr(72, 3, ResolvedExprKind::Id(x))),
                            Box::new(expr(76, 4, ResolvedExprKind::Int(1))),
                        ),
                    ),
                )],
            ),
        ]
    );
}

#[test]
fn resolves_mutually_recursive_functions() {
    let mut expected_ctx = ResolverCtx::new();
    let even = expected_ctx.add_value("even");
    let odd = expected_ctx.add_value("odd");
    let main = expected_ctx.add_value("main");
    let even_parameter = expected_ctx.add_value("x");
    let odd_parameter = expected_ctx.add_value("x");

    assert_eq!(
        resolve(
            "fn even(x: Int) -> Bool { return if x == 0 then true else odd(x - 1); } \
             fn odd(x: Int) -> Bool { return if x == 0 then false else even(x - 1); } \
             fn main() -> Int { return 0; }",
        ),
        vec![
            ResolvedDefn::mk_fn(
                0,
                even,
                vec![ResolvedBinding::new(
                    8,
                    even_parameter,
                    ResolvedType::mk_int(11),
                )],
                ResolvedType::mk_bool(19),
                vec![recursive_return(
                    26,
                    33,
                    36,
                    41,
                    48,
                    58,
                    62,
                    66,
                    0,
                    even_parameter,
                    odd,
                    true,
                )],
            ),
            ResolvedDefn::mk_fn(
                72,
                odd,
                vec![ResolvedBinding::new(
                    79,
                    odd_parameter,
                    ResolvedType::mk_int(82),
                )],
                ResolvedType::mk_bool(90),
                vec![recursive_return(
                    97,
                    104,
                    107,
                    112,
                    119,
                    130,
                    135,
                    139,
                    10,
                    odd_parameter,
                    even,
                    false,
                )],
            ),
            ResolvedDefn::mk_fn(
                145,
                main,
                vec![],
                ResolvedType::mk_int(158),
                vec![ResolvedStmt::mk_return(
                    164,
                    expr(171, 20, ResolvedExprKind::Int(0)),
                )],
            ),
        ]
    );
}

fn recursive_return(
    return_offset: usize,
    if_offset: usize,
    condition_offset: usize,
    zero_offset: usize,
    bool_offset: usize,
    call_offset: usize,
    argument_offset: usize,
    one_offset: usize,
    first_expr_id: usize,
    parameter: super::types::ValueId,
    function: super::types::ValueId,
    bool_value: bool,
) -> ResolvedStmt {
    ResolvedStmt::mk_return(
        return_offset,
        expr(
            if_offset,
            first_expr_id + 9,
            ResolvedExprKind::If(
                Box::new(expr(
                    condition_offset,
                    first_expr_id + 2,
                    ResolvedExprKind::Eq(
                        Box::new(expr(
                            condition_offset,
                            first_expr_id,
                            ResolvedExprKind::Id(parameter),
                        )),
                        Box::new(expr(
                            zero_offset,
                            first_expr_id + 1,
                            ResolvedExprKind::Int(0),
                        )),
                    ),
                )),
                Box::new(expr(
                    bool_offset,
                    first_expr_id + 3,
                    ResolvedExprKind::Bool(bool_value),
                )),
                Box::new(expr(
                    call_offset,
                    first_expr_id + 8,
                    ResolvedExprKind::Call(
                        Box::new(expr(
                            call_offset,
                            first_expr_id + 4,
                            ResolvedExprKind::Id(function),
                        )),
                        vec![expr(
                            argument_offset,
                            first_expr_id + 7,
                            ResolvedExprKind::Minus(
                                Box::new(expr(
                                    argument_offset,
                                    first_expr_id + 5,
                                    ResolvedExprKind::Id(parameter),
                                )),
                                Box::new(expr(
                                    one_offset,
                                    first_expr_id + 6,
                                    ResolvedExprKind::Int(1),
                                )),
                            ),
                        )],
                    ),
                )),
            ),
        ),
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
    let a_value = expected_ctx.add_value("value");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    let b_next = expected_ctx.add_value("next");
    expected_ctx.pop_scope();
    expected_ctx.pop_scope();

    expected_ctx.push_scope();
    let b_type = expected_ctx.add_typeparam("'b");
    expected_ctx.push_scope();
    let b_value = expected_ctx.add_value("value");
    expected_ctx.pop_scope();
    expected_ctx.push_scope();
    let a_next = expected_ctx.add_value("next");
    expected_ctx.pop_scope();
    expected_ctx.pop_scope();

    expected_ctx.push_scope();
    expected_ctx.pop_scope();

    assert_eq!(
        resolved,
        vec![
            ResolvedDefn::mk_typedef(
                0,
                recursive_a,
                vec![a_type],
                vec![
                    ResolvedConstructorArm::new(
                        25,
                        a,
                        vec![ResolvedBinding::new(
                            27,
                            a_value,
                            ResolvedType::mk_typeparam(34, a_type),
                        )],
                    ),
                    ResolvedConstructorArm::new(
                        39,
                        b,
                        vec![ResolvedBinding::new(
                            41,
                            b_next,
                            ResolvedType::mk_typeapp(
                                47,
                                recursive_b,
                                vec![ResolvedType::mk_typeparam(58, a_type)],
                            ),
                        )],
                    ),
                ],
            ),
            ResolvedDefn::mk_typedef(
                65,
                recursive_b,
                vec![b_type],
                vec![
                    ResolvedConstructorArm::new(
                        90,
                        c,
                        vec![ResolvedBinding::new(
                            92,
                            b_value,
                            ResolvedType::mk_typeparam(99, b_type),
                        )],
                    ),
                    ResolvedConstructorArm::new(
                        104,
                        d,
                        vec![ResolvedBinding::new(
                            106,
                            a_next,
                            ResolvedType::mk_typeapp(
                                112,
                                recursive_a,
                                vec![ResolvedType::mk_typeparam(123, b_type)],
                            ),
                        )],
                    ),
                ],
            ),
            ResolvedDefn::mk_fn(
                130,
                main,
                vec![],
                ResolvedType::mk_int(143),
                vec![ResolvedStmt::mk_return(
                    149,
                    expr(156, 0, ResolvedExprKind::Int(0)),
                )],
            ),
        ]
    );
}

#[test]
fn rejects_empty_program() {
    assert_resolution_error(
        "",
        Spanned {
            offset: 0,
            node: ResolutionErrorKind::MissingMainFn,
        },
    );
}

#[test]
fn rejects_program_without_main_function() {
    assert_resolution_error(
        "fn helper() -> Int { return 0; }",
        Spanned {
            offset: 0,
            node: ResolutionErrorKind::MissingMainFn,
        },
    );
}

#[test]
fn rejects_typedef_only_program() {
    assert_resolution_error(
        "typedef Unit { unit() }",
        Spanned {
            offset: 0,
            node: ResolutionErrorKind::MissingMainFn,
        },
    );
}

#[test]
fn rejects_duplicate_function_name() {
    assert_resolution_error(
        "fn duplicate() -> Int { return 0; } \
         fn duplicate() -> Int { return 1; } \
         fn main() -> Int { return 0; }",
        Spanned {
            offset: 36,
            node: ResolutionErrorKind::DuplicateValue { name: "duplicate" },
        },
    );
}

#[test]
fn rejects_duplicate_function_name_with_different_parameters() {
    assert_resolution_error(
        "fn duplicate() -> Int { return 0; } \
         fn duplicate(x: Int) -> Int { return x; } \
         fn main() -> Int { return 0; }",
        Spanned {
            offset: 36,
            node: ResolutionErrorKind::DuplicateValue { name: "duplicate" },
        },
    );
}

#[test]
fn rejects_function_and_constructor_name_collision() {
    assert_resolution_error(
        "fn duplicate() -> Int { return 0; } \
         typedef Duplicate { duplicate() } \
         fn main() -> Int { return 0; }",
        Spanned {
            offset: 56,
            node: ResolutionErrorKind::DuplicateValue { name: "duplicate" },
        },
    );
}

#[test]
fn rejects_function_and_constructor_name_collision_with_different_parameters() {
    assert_resolution_error(
        "fn duplicate() -> Int { return 0; } \
         typedef Duplicate { duplicate(value: Int) } \
         fn main() -> Int { return 0; }",
        Spanned {
            offset: 56,
            node: ResolutionErrorKind::DuplicateValue { name: "duplicate" },
        },
    );
}

#[test]
fn rejects_duplicate_typedef_name() {
    assert_resolution_error(
        "typedef Duplicate { first() } \
         typedef Duplicate { second() } \
         fn main() -> Int { return 0; }",
        Spanned {
            offset: 30,
            node: ResolutionErrorKind::DuplicateType { name: "Duplicate" },
        },
    );
}

#[test]
fn rejects_duplicate_typedef_name_with_different_type_parameters() {
    assert_resolution_error(
        "typedef Duplicate { first() } \
         typedef Duplicate('a) { second(value: 'a) } \
         fn main() -> Int { return 0; }",
        Spanned {
            offset: 30,
            node: ResolutionErrorKind::DuplicateType { name: "Duplicate" },
        },
    );
}

#[test]
fn rejects_nullary_constructor_name_collision_across_typedefs() {
    assert_resolution_error(
        "typedef First { duplicate() } \
         typedef Second { duplicate() } \
         fn main() -> Int { return 0; }",
        Spanned {
            offset: 47,
            node: ResolutionErrorKind::DuplicateValue { name: "duplicate" },
        },
    );
}

#[test]
fn rejects_constructor_name_collision_across_typedefs() {
    assert_resolution_error(
        "typedef First { duplicate() } \
         typedef Second { duplicate(value: Int) } \
         fn main() -> Int { return 0; }",
        Spanned {
            offset: 47,
            node: ResolutionErrorKind::DuplicateValue { name: "duplicate" },
        },
    );
}

#[test]
fn same_generic_name_in_separate_functions_gets_distinct_ids() {
    let mut expected_ctx = ResolverCtx::new();
    let first = expected_ctx.add_value("first");
    let second = expected_ctx.add_value("second");
    let main = expected_ctx.add_value("main");
    let first_type = expected_ctx.add_typeparam("'a");
    let first_x = expected_ctx.add_value("x");
    let second_type = expected_ctx.add_typeparam("'a");
    let second_x = expected_ctx.add_value("x");

    assert_eq!(
        resolve(
            "fn first(x: 'a) -> 'a { return x; } \
             fn second(x: 'a) -> 'a { return x; } \
             fn main() -> Int { return 0; }",
        ),
        vec![
            ResolvedDefn::mk_fn(
                0,
                first,
                vec![ResolvedBinding::new(
                    9,
                    first_x,
                    ResolvedType::mk_typeparam(12, first_type),
                )],
                ResolvedType::mk_typeparam(19, first_type),
                vec![ResolvedStmt::mk_return(
                    24,
                    expr(31, 0, ResolvedExprKind::Id(first_x)),
                )],
            ),
            ResolvedDefn::mk_fn(
                36,
                second,
                vec![ResolvedBinding::new(
                    46,
                    second_x,
                    ResolvedType::mk_typeparam(49, second_type),
                )],
                ResolvedType::mk_typeparam(56, second_type),
                vec![ResolvedStmt::mk_return(
                    61,
                    expr(68, 1, ResolvedExprKind::Id(second_x)),
                )],
            ),
            ResolvedDefn::mk_fn(
                73,
                main,
                vec![],
                ResolvedType::mk_int(86),
                vec![ResolvedStmt::mk_return(
                    92,
                    expr(99, 2, ResolvedExprKind::Int(0)),
                )],
            ),
        ]
    );
}

#[test]
fn resolves_all_definition_kinds_independent_of_source_order() {
    let input = "fn main() -> Int { return make(1); } \
                 typedef Box { make(value: Int) }";

    let result = resolve_result(input);
    assert!(
        result.is_ok(),
        "unexpected errors: {:?}",
        result.err().map(|failure| failure.errors)
    );
}

#[test]
fn resolves_forward_type_references_in_function_signatures() {
    let input = "fn unwrap(box: Box) -> Box { return box; } \
                 typedef Box { box(value: Int) } \
                 fn main() -> Int { return 0; }";

    let result = resolve_result(input);
    assert!(
        result.is_ok(),
        "unexpected errors: {:?}",
        result.err().map(|failure| failure.errors)
    );
}

#[test]
fn reports_unknown_type_in_function_return_type_at_the_type_offset() {
    let input = "fn main() -> Missing { return 0; }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 13,
            node: ResolutionErrorKind::UnknownType { name: "Missing" },
        }]
    );
}

#[test]
fn reports_unknown_type_in_typedef_type_application_without_panicking() {
    let input = "typedef Box { make(value: Missing(Int)) } \
                 fn main() -> Int { return 0; }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 26,
            node: ResolutionErrorKind::UnknownType { name: "Missing" },
        }]
    );
}

#[test]
fn reports_unknown_type_nested_in_a_typedef_function_type() {
    let input = "typedef Box { make(value: Int -> Missing) } \
                 fn main() -> Int { return 0; }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 33,
            node: ResolutionErrorKind::UnknownType { name: "Missing" },
        }]
    );
}

#[test]
fn reports_each_duplicate_global_constructor_name() {
    let input = "typedef First { make() } \
                 typedef Second { make() } \
                 fn main() -> Int { return 0; }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 42,
            node: ResolutionErrorKind::DuplicateValue { name: "make" },
        }]
    );
}

#[test]
fn reports_duplicate_constructor_field_names_at_the_second_field() {
    let input = "typedef Pair { pair(left: Int, left: Bool) } \
                 fn main() -> Int { return 0; }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 31,
            node: ResolutionErrorKind::DuplicateValue { name: "left" },
        }]
    );
}

#[test]
fn reports_duplicate_match_binding_names() {
    let input = "typedef Option { some(value: Int), none() } \
                 fn main() -> Int { \
                    let option = some(1); \
                    return match option { some(value, value) -> value, none() -> 0 }; \
                 }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 107,
            node: ResolutionErrorKind::DuplicateValue { name: "value" },
        }]
    );
}

#[test]
fn let_rhs_resolves_before_the_new_binding_is_added() {
    let input = "fn main(x: Int) -> Int { let x = x; return x; }";

    assert!(resolve_result(input).is_ok());
}

#[test]
fn let_and_read_bindings_shadow_parameters_and_each_other() {
    let input = "fn main(x: Int) -> Int { \
                    let x = 1; \
                    read Int x; \
                    let x = x; \
                    return x; \
                 }";

    assert!(resolve_result(input).is_ok());
}

#[test]
fn closure_parameter_does_not_leak_into_the_enclosing_function() {
    let input = "fn main() -> Int { \
                    let f = fn (x: Int) -> x; \
                    return x; \
                 }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 52,
            node: ResolutionErrorKind::UnknownValue { name: "x" },
        }]
    );
}

#[test]
fn match_bindings_do_not_leak_between_arms_or_after_the_match() {
    let input = "typedef Option { some(value: Int), none() } \
                 fn main() -> Int { \
                    let option = some(1); \
                    let result = match option { \
                        some(value) -> value, \
                        none() -> value \
                    }; \
                    return value; \
                 }";

    assert_eq!(
        errors(input),
        vec![
            Spanned {
                offset: 145,
                node: ResolutionErrorKind::UnknownValue { name: "value" },
            },
            Spanned {
                offset: 161,
                node: ResolutionErrorKind::UnknownValue { name: "value" },
            },
        ]
    );
}

#[test]
fn unknown_match_variant_is_reported() {
    let input = "fn main() -> Int { return match 0 { missing() -> 0 }; }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 36,
            node: ResolutionErrorKind::UnknownValue { name: "missing" },
        }]
    );
}

#[test]
fn unknown_identifier_is_reported_at_each_distinct_source_use() {
    let input = "fn main() -> Int { return missing + other; }";

    assert_eq!(
        errors(input),
        vec![
            Spanned {
                offset: 26,
                node: ResolutionErrorKind::UnknownValue { name: "missing" },
            },
            Spanned {
                offset: 36,
                node: ResolutionErrorKind::UnknownValue { name: "other" },
            },
        ]
    );
}

#[test]
fn missing_main_is_reported_alongside_other_resolution_errors() {
    let input = "fn helper() -> Int { return missing; }";

    assert_eq!(
        errors(input),
        vec![
            Spanned {
                offset: 0,
                node: ResolutionErrorKind::MissingMainFn,
            },
            Spanned {
                offset: 28,
                node: ResolutionErrorKind::UnknownValue { name: "missing" },
            },
        ]
    );
}

#[test]
fn error_recovery_does_not_create_duplicate_errors_for_repeated_unknown_type_parameter() {
    let input = "typedef Box('a) { make(left: 'missing, right: 'missing) } \
                 fn main() -> Int { return 0; }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 29,
            node: ResolutionErrorKind::UnknownType { name: "'missing" },
        }]
    );
}

#[test]
fn errors_in_one_definition_do_not_prevent_later_definitions_from_being_resolved() {
    let input = "fn bad() -> Int { return missing; } \
                 fn main() -> Int { return later(0); } \
                 fn later(x: Int) -> Int { return x; }";

    assert_eq!(
        errors(input),
        vec![Spanned {
            offset: 25,
            node: ResolutionErrorKind::UnknownValue { name: "missing" },
        }]
    );
}
