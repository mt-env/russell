use crate::frontend::{
    lexer::lex,
    parser::{
        Parser,
        ast::{ParsedBinding, Type},
        parse_type::parse_type,
    },
    resolution::types::{ResolvedBinding, ResolvedType, ResolverCtx},
};

fn resolve_type<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedType {
    let mut parser = Parser::new(lex(input));
    let parsed = parse_type(&mut parser).unwrap();
    super::add_type(ctx, parsed)
}

fn resolve_binding<'a>(
    ctx: &mut ResolverCtx<'a>,
    name: &'a str,
    type_source: &'a str,
) -> ResolvedBinding {
    let mut parser = Parser::new(lex(type_source));
    let parsed = ParsedBinding::new(0, name, parse_type(&mut parser).unwrap());
    super::add_binding(ctx, parsed)
}

#[test]
fn resolves_primitive_types() {
    assert_eq!(
        resolve_type(&mut ResolverCtx::new(), "Int"),
        ResolvedType::Int
    );
    assert_eq!(
        resolve_type(&mut ResolverCtx::new(), "Float"),
        ResolvedType::Float
    );
    assert_eq!(
        resolve_type(&mut ResolverCtx::new(), "Bool"),
        ResolvedType::Bool
    );
}

#[test]
fn resolves_named_type() {
    let mut ctx = ResolverCtx::new();
    let option = ctx.add_type("Option");

    assert_eq!(
        resolve_type(&mut ctx, "Option"),
        ResolvedType::TypeId(option)
    );
}

#[test]
#[should_panic(expected = "TypeId Missing not found in context")]
fn rejects_unknown_named_type() {
    resolve_type(&mut ResolverCtx::new(), "Missing");
}

#[test]
fn resolves_function_type_right_associatively() {
    assert_eq!(
        resolve_type(&mut ResolverCtx::new(), "Int -> Float -> Bool"),
        ResolvedType::Fn(
            Box::new(ResolvedType::Int),
            Box::new(ResolvedType::Fn(
                Box::new(ResolvedType::Float),
                Box::new(ResolvedType::Bool),
            )),
        )
    );
}

#[test]
fn resolves_type_parameter_and_reuses_it() {
    let mut ctx = ResolverCtx::new();
    let expected = ctx.add_typeparam("'a");

    assert_eq!(
        resolve_type(&mut ctx, "'a -> 'a"),
        ResolvedType::Fn(
            Box::new(ResolvedType::TypeParam(expected)),
            Box::new(ResolvedType::TypeParam(expected)),
        )
    );
}

#[test]
fn creates_type_parameter_when_it_is_not_in_scope() {
    let mut ctx = ResolverCtx::new();
    let resolved = resolve_type(&mut ctx, "'a");

    assert_eq!(
        resolved,
        ResolvedType::TypeParam(ctx.lookup_typeparam("'a").unwrap())
    );
}

#[test]
fn resolves_type_application() {
    let mut ctx = ResolverCtx::new();
    let result = ctx.add_type("Result");

    assert_eq!(
        resolve_type(&mut ctx, "Result(Int, Bool)"),
        ResolvedType::TypeApp(result, vec![ResolvedType::Int, ResolvedType::Bool])
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
        ResolvedType::TypeApp(
            map,
            vec![
                ResolvedType::Int,
                ResolvedType::TypeApp(list, vec![ResolvedType::TypeParam(parameter)],),
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
        resolve_binding(&mut ctx, "x", "Int"),
        ResolvedBinding::new(expected, ResolvedType::Int)
    );
    assert_eq!(ctx.lookup_value("x"), Some(expected));
}

#[test]
fn resolves_binding_with_function_type() {
    let mut ctx = ResolverCtx::new();
    let mut expected_ctx = ResolverCtx::new();
    let expected = expected_ctx.add_value("f");

    assert_eq!(
        resolve_binding(&mut ctx, "f", "Int -> Float"),
        ResolvedBinding::new(
            expected,
            ResolvedType::Fn(Box::new(ResolvedType::Int), Box::new(ResolvedType::Float),),
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
        resolve_binding(&mut ctx, "value", "Result(Int, Bool)"),
        ResolvedBinding::new(
            value,
            ResolvedType::TypeApp(result, vec![ResolvedType::Int, ResolvedType::Bool]),
        )
    );
}

#[test]
fn existing_type_parameter_binding_reuses_parameter() {
    let mut ctx = ResolverCtx::new();
    let parameter = ctx.add_typeparam("'a");
    let binding = ParsedBinding::new(0, "value", Type::TypeParam("'a"));

    let resolved = super::add_binding_existing_typaram(&mut ctx, binding).unwrap();
    let value = ctx.lookup_value("value").unwrap();

    assert_eq!(
        resolved,
        ResolvedBinding::new(value, ResolvedType::TypeParam(parameter))
    );
}

#[test]
fn existing_type_parameter_binding_rejects_unknown_parameter() {
    let mut ctx = ResolverCtx::new();
    let binding = ParsedBinding::new(0, "value", Type::TypeParam("'missing"));

    assert_eq!(super::add_binding_existing_typaram(&mut ctx, binding), None);
}

#[test]
fn existing_type_parameter_binding_resolves_primitive_types() {
    let mut ctx = ResolverCtx::new();

    let int_binding = ParsedBinding::new(0, "integer", Type::Int);
    let integer = super::add_binding_existing_typaram(&mut ctx, int_binding).unwrap();
    assert_eq!(
        integer,
        ResolvedBinding::new(ctx.lookup_value("integer").unwrap(), ResolvedType::Int)
    );

    let float_binding = ParsedBinding::new(0, "float", Type::Float);
    let float = super::add_binding_existing_typaram(&mut ctx, float_binding).unwrap();
    assert_eq!(
        float,
        ResolvedBinding::new(ctx.lookup_value("float").unwrap(), ResolvedType::Float)
    );

    let bool_binding = ParsedBinding::new(0, "boolean", Type::Bool);
    let boolean = super::add_binding_existing_typaram(&mut ctx, bool_binding).unwrap();
    assert_eq!(
        boolean,
        ResolvedBinding::new(ctx.lookup_value("boolean").unwrap(), ResolvedType::Bool)
    );
}

#[test]
fn existing_type_parameter_binding_resolves_named_type() {
    let mut ctx = ResolverCtx::new();
    let option = ctx.add_type("Option");
    let binding = ParsedBinding::new(0, "value", Type::TypeId("Option"));

    let resolved = super::add_binding_existing_typaram(&mut ctx, binding).unwrap();

    assert_eq!(
        resolved,
        ResolvedBinding::new(
            ctx.lookup_value("value").unwrap(),
            ResolvedType::TypeId(option),
        )
    );
}

#[test]
fn existing_type_parameter_binding_resolves_type_application() {
    let mut ctx = ResolverCtx::new();
    let option = ctx.add_type("Option");
    let binding = ParsedBinding::new(0, "value", Type::TypeApp("Option", vec![Type::Int]));

    let resolved = super::add_binding_existing_typaram(&mut ctx, binding).unwrap();

    assert_eq!(
        resolved,
        ResolvedBinding::new(
            ctx.lookup_value("value").unwrap(),
            ResolvedType::TypeApp(option, vec![ResolvedType::Int]),
        )
    );
}

#[test]
fn existing_type_parameter_binding_resolves_function_type() {
    let mut ctx = ResolverCtx::new();
    let binding = ParsedBinding::new(
        0,
        "function",
        Type::Fn(Box::new(Type::Int), Box::new(Type::Bool)),
    );

    let resolved = super::add_binding_existing_typaram(&mut ctx, binding).unwrap();

    assert_eq!(
        resolved,
        ResolvedBinding::new(
            ctx.lookup_value("function").unwrap(),
            ResolvedType::Fn(Box::new(ResolvedType::Int), Box::new(ResolvedType::Bool)),
        )
    );
}
