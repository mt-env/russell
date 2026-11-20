use crate::frontend::{
    lexer::lex,
    parser::{
        Parser,
        ast::ParsedBinding,
        parse_type::{parse_binding, parse_type},
    },
    resolution::types::{
        ResolveErrorKind, ResolveResult, ResolvedBinding, ResolvedType, ResolverCtx,
    },
    types::Spanned,
};

fn resolve_type_result<'a>(
    ctx: &mut ResolverCtx<'a>,
    input: &'a str,
) -> ResolveResult<'a, ResolvedType> {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    let parsed = parse_type(&mut parser).unwrap();
    super::add_type(ctx, parsed)
}

fn resolve_type<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedType {
    resolve_type_result(ctx, input).expect("test input should resolve successfully")
}

fn parse_test_binding(input: &str) -> ParsedBinding<'_> {
    let mut parser = Parser::new(lex(input).expect("test input should lex successfully"));
    parse_binding(&mut parser).expect("test input should parse successfully")
}

fn resolve_binding<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedBinding {
    let parsed = parse_test_binding(input);
    super::add_binding(ctx, parsed).expect("test input should resolve successfully")
}

#[test]
fn resolves_primitive_types() {
    assert_eq!(
        resolve_type(&mut ResolverCtx::new(), "Int"),
        ResolvedType::mk_int(0)
    );
    assert_eq!(
        resolve_type(&mut ResolverCtx::new(), "Float"),
        ResolvedType::mk_float(0)
    );
    assert_eq!(
        resolve_type(&mut ResolverCtx::new(), "Bool"),
        ResolvedType::mk_bool(0)
    );
}

#[test]
fn resolves_named_type() {
    let mut ctx = ResolverCtx::new();
    let option = ctx.add_type("Option");

    assert_eq!(
        resolve_type(&mut ctx, "Option"),
        ResolvedType::mk_typeid(0, option)
    );
}

#[test]
fn rejects_unknown_named_type() {
    let mut ctx = ResolverCtx::new();
    assert_eq!(
        resolve_type_result(&mut ctx, "Missing"),
        Err(Spanned {
            offset: 0,
            node: ResolveErrorKind::UnknownType { name: "Missing" },
        })
    );
}

#[test]
fn resolves_function_type_right_associatively() {
    assert_eq!(
        resolve_type(&mut ResolverCtx::new(), "Int -> Float -> Bool"),
        ResolvedType::mk_fn(
            0,
            ResolvedType::mk_int(0),
            ResolvedType::mk_fn(7, ResolvedType::mk_float(7), ResolvedType::mk_bool(16),),
        )
    );
}

#[test]
fn resolves_type_parameter_and_reuses_it() {
    let mut ctx = ResolverCtx::new();
    let expected = ctx.add_typeparam("'a");

    assert_eq!(
        resolve_type(&mut ctx, "'a -> 'a"),
        ResolvedType::mk_fn(
            0,
            ResolvedType::mk_typeparam(0, expected),
            ResolvedType::mk_typeparam(6, expected),
        )
    );
}

#[test]
fn creates_type_parameter_when_it_is_not_in_scope() {
    let mut ctx = ResolverCtx::new();
    let resolved = resolve_type(&mut ctx, "'a");

    assert_eq!(
        resolved,
        ResolvedType::mk_typeparam(0, ctx.lookup_typeparam("'a").unwrap())
    );
}

#[test]
fn resolves_type_application() {
    let mut ctx = ResolverCtx::new();
    let result = ctx.add_type("Result");

    assert_eq!(
        resolve_type(&mut ctx, "Result(Int, Bool)"),
        ResolvedType::mk_typeapp(
            0,
            result,
            vec![ResolvedType::mk_int(7), ResolvedType::mk_bool(12)]
        )
    );
}

#[test]
fn resolves_nested_type_application() {
    let mut ctx = ResolverCtx::new();
    let map = ctx.add_type("Map");
    let list = ctx.add_type("List");
    let parameter = ctx.add_typeparam("'a");

    assert_eq!(
        resolve_type(&mut ctx, "Map(Int, List('a))"),
        ResolvedType::mk_typeapp(
            0,
            map,
            vec![
                ResolvedType::mk_int(4),
                ResolvedType::mk_typeapp(9, list, vec![ResolvedType::mk_typeparam(14, parameter)]),
            ],
        )
    );
}

#[test]
fn resolves_binding_and_registers_its_name() {
    let mut ctx = ResolverCtx::new();
    let mut expected_ctx = ResolverCtx::new();
    let expected = expected_ctx.add_value("x");

    assert_eq!(
        resolve_binding(&mut ctx, "x: Int"),
        ResolvedBinding::new(0, expected, ResolvedType::mk_int(3))
    );
    assert_eq!(ctx.lookup_value("x"), Some(expected));
}

#[test]
fn resolves_binding_with_function_type() {
    let mut ctx = ResolverCtx::new();
    let mut expected_ctx = ResolverCtx::new();
    let expected = expected_ctx.add_value("f");

    assert_eq!(
        resolve_binding(&mut ctx, "f: Int -> Float"),
        ResolvedBinding::new(
            0,
            expected,
            ResolvedType::mk_fn(3, ResolvedType::mk_int(3), ResolvedType::mk_float(10),),
        )
    );
}

#[test]
fn resolves_binding_with_type_application() {
    let mut ctx = ResolverCtx::new();
    let result = ctx.add_type("Result");
    let mut expected_ctx = ResolverCtx::new();
    expected_ctx.add_type("Result");
    let value = expected_ctx.add_value("value");

    assert_eq!(
        resolve_binding(&mut ctx, "value: Result(Int, Bool)"),
        ResolvedBinding::new(
            0,
            value,
            ResolvedType::mk_typeapp(
                7,
                result,
                vec![ResolvedType::mk_int(14), ResolvedType::mk_bool(19)]
            ),
        )
    );
}

#[test]
fn existing_type_parameter_binding_reuses_parameter() {
    let mut ctx = ResolverCtx::new();
    let parameter = ctx.add_typeparam("'a");
    let binding = parse_test_binding("value: 'a");

    let resolved = super::add_binding_existing_typaram(&mut ctx, binding).unwrap();
    let value = ctx.lookup_value("value").unwrap();

    assert_eq!(
        resolved,
        ResolvedBinding::new(0, value, ResolvedType::mk_typeparam(7, parameter))
    );
}

#[test]
fn existing_type_parameter_binding_rejects_unknown_parameter() {
    let mut ctx = ResolverCtx::new();
    let binding = parse_test_binding("value: 'missing");

    assert_eq!(
        super::add_binding_existing_typaram(&mut ctx, binding),
        Err(Spanned {
            offset: 7,
            node: ResolveErrorKind::UnknownType { name: "'missing" },
        })
    );
    assert_eq!(ctx.lookup_value("value"), None);
}

#[test]
fn existing_type_parameter_binding_resolves_primitive_types() {
    let mut ctx = ResolverCtx::new();

    let int_binding = parse_test_binding("integer: Int");
    let integer = super::add_binding_existing_typaram(&mut ctx, int_binding).unwrap();
    assert_eq!(
        integer,
        ResolvedBinding::new(
            0,
            ctx.lookup_value("integer").unwrap(),
            ResolvedType::mk_int(9)
        )
    );

    let float_binding = parse_test_binding("float: Float");
    let float = super::add_binding_existing_typaram(&mut ctx, float_binding).unwrap();
    assert_eq!(
        float,
        ResolvedBinding::new(
            0,
            ctx.lookup_value("float").unwrap(),
            ResolvedType::mk_float(7)
        )
    );

    let bool_binding = parse_test_binding("boolean: Bool");
    let boolean = super::add_binding_existing_typaram(&mut ctx, bool_binding).unwrap();
    assert_eq!(
        boolean,
        ResolvedBinding::new(
            0,
            ctx.lookup_value("boolean").unwrap(),
            ResolvedType::mk_bool(9)
        )
    );
}

#[test]
fn existing_type_parameter_binding_resolves_named_type() {
    let mut ctx = ResolverCtx::new();
    let option = ctx.add_type("Option");
    let binding = parse_test_binding("value: Option");

    let resolved = super::add_binding_existing_typaram(&mut ctx, binding).unwrap();

    assert_eq!(
        resolved,
        ResolvedBinding::new(
            0,
            ctx.lookup_value("value").unwrap(),
            ResolvedType::mk_typeid(7, option),
        )
    );
}

#[test]
fn existing_type_parameter_binding_resolves_type_application() {
    let mut ctx = ResolverCtx::new();
    let option = ctx.add_type("Option");
    let binding = parse_test_binding("value: Option(Int)");

    let resolved = super::add_binding_existing_typaram(&mut ctx, binding).unwrap();

    assert_eq!(
        resolved,
        ResolvedBinding::new(
            0,
            ctx.lookup_value("value").unwrap(),
            ResolvedType::mk_typeapp(7, option, vec![ResolvedType::mk_int(14)]),
        )
    );
}

#[test]
fn existing_type_parameter_binding_resolves_function_type() {
    let mut ctx = ResolverCtx::new();
    let binding = parse_test_binding("function: Int -> Bool");

    let resolved = super::add_binding_existing_typaram(&mut ctx, binding).unwrap();

    assert_eq!(
        resolved,
        ResolvedBinding::new(
            0,
            ctx.lookup_value("function").unwrap(),
            ResolvedType::mk_fn(10, ResolvedType::mk_int(10), ResolvedType::mk_bool(17),),
        )
    );
}
