use crate::frontend::lexer::lex;
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::{ParsedBinding, Type};

fn parser_from(input: &str) -> Parser<'_> {
    Parser::new(lex(input))
}

fn parse(input: &str) -> Type<'_> {
    let mut p = parser_from(input);
    super::parse_type(&mut p).unwrap()
}

// ─── parse_type ─────────────────────────────────────────────────────

#[test]
fn int_type() {
    assert_eq!(parse("Int"), Type::Int);
}

#[test]
fn float_type() {
    assert_eq!(parse("Float"), Type::Float);
}

#[test]
fn bool_type() {
    assert_eq!(parse("Bool"), Type::Bool);
}

#[test]
fn typeid() {
    assert_eq!(parse("MyType"), Type::TypeId("MyType".into()));
}

#[test]
fn fn_type_simple() {
    assert_eq!(
        parse("Int -> Float"),
        Type::Fn(Box::new(Type::Int), Box::new(Type::Float))
    );
}

#[test]
fn fn_type_right_associative() {
    // Int -> Float -> Bool = Int -> (Float -> Bool)
    assert_eq!(
        parse("Int -> Float -> Bool"),
        Type::Fn(
            Box::new(Type::Int),
            Box::new(Type::Fn(Box::new(Type::Float), Box::new(Type::Bool)))
        )
    );
}

#[test]
fn fn_type_three_deep() {
    // Int -> Float -> Bool -> Int = Int -> (Float -> (Bool -> Int))
    assert_eq!(
        parse("Int -> Float -> Bool -> Int"),
        Type::Fn(
            Box::new(Type::Int),
            Box::new(Type::Fn(
                Box::new(Type::Float),
                Box::new(Type::Fn(Box::new(Type::Bool), Box::new(Type::Int)))
            ))
        )
    );
}

#[test]
fn fn_type_with_typeid() {
    assert_eq!(
        parse("MyType -> Int"),
        Type::Fn(Box::new(Type::TypeId("MyType".into())), Box::new(Type::Int))
    );
}

#[test]
fn error_on_int_literal() {
    let mut p = parser_from("42");
    assert!(super::parse_type(&mut p).is_err());
}

#[test]
fn error_on_operator() {
    let mut p = parser_from("+");
    assert!(super::parse_type(&mut p).is_err());
}

#[test]
fn error_on_identifier() {
    let mut p = parser_from("foo");
    assert!(super::parse_type(&mut p).is_err());
}

// ─── parse_binding ──────────────────────────────────────────────────

#[test]
fn simple_binding() {
    let mut p = parser_from("x : Int");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(b, ParsedBinding::new(0, "x".into(), Type::Int));
}

#[test]
fn binding_with_float_type() {
    let mut p = parser_from("val : Float");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(b, ParsedBinding::new(0, "val".into(), Type::Float));
}

#[test]
fn binding_with_bool_type() {
    let mut p = parser_from("flag : Bool");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(b, ParsedBinding::new(0, "flag".into(), Type::Bool));
}

#[test]
fn binding_with_fn_type() {
    let mut p = parser_from("f : Int -> Bool");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(
        b,
        ParsedBinding::new(0, "f".into(), Type::Fn(Box::new(Type::Int), Box::new(Type::Bool)))
    );
}

#[test]
fn binding_with_typeid() {
    let mut p = parser_from("x : MyType");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(b, ParsedBinding::new(0, "x".into(), Type::TypeId("MyType".into())));
}

#[test]
fn binding_error_missing_colon() {
    let mut p = parser_from("x Int");
    assert!(super::parse_binding(&mut p).is_err());
}

#[test]
fn binding_error_missing_id() {
    let mut p = parser_from("42 : Int");
    assert!(super::parse_binding(&mut p).is_err());
}

#[test]
fn binding_error_missing_type() {
    let mut p = parser_from("x : +");
    assert!(super::parse_binding(&mut p).is_err());
}

// ─── parse_binding_list ─────────────────────────────────────────────

#[test]
fn empty_binding_list() {
    let mut p = parser_from("()");
    let bindings = super::parse_binding_list(&mut p).unwrap();
    assert!(bindings.is_empty());
}

#[test]
fn single_binding_list() {
    let mut p = parser_from("(x : Int)");
    let bindings = super::parse_binding_list(&mut p).unwrap();
    assert_eq!(bindings, vec![ParsedBinding::new(1, "x".into(), Type::Int)]);
}

#[test]
fn multiple_bindings() {
    let mut p = parser_from("(x : Int, y : Float)");
    let bindings = super::parse_binding_list(&mut p).unwrap();
    assert_eq!(
        bindings,
        vec![
            ParsedBinding::new(1, "x".into(), Type::Int),
            ParsedBinding::new(10, "y".into(), Type::Float),
        ]
    );
}

#[test]
fn three_bindings() {
    let mut p = parser_from("(a : Int, b : Float, c : Bool)");
    let bindings = super::parse_binding_list(&mut p).unwrap();
    assert_eq!(
        bindings,
        vec![
            ParsedBinding::new(1, "a".into(), Type::Int),
            ParsedBinding::new(10, "b".into(), Type::Float),
            ParsedBinding::new(21, "c".into(), Type::Bool),
        ]
    );
}

#[test]
fn binding_list_with_fn_type() {
    let mut p = parser_from("(f : Int -> Bool, x : Int)");
    let bindings = super::parse_binding_list(&mut p).unwrap();
    assert_eq!(
        bindings,
        vec![
            ParsedBinding::new(1, "f".into(), Type::Fn(Box::new(Type::Int), Box::new(Type::Bool))),
            ParsedBinding::new(18, "x".into(), Type::Int),
        ]
    );
}

#[test]
fn binding_list_error_missing_lparen() {
    let mut p = parser_from("x : Int)");
    assert!(super::parse_binding_list(&mut p).is_err());
}

#[test]
fn binding_list_error_missing_rparen() {
    let mut p = parser_from("(x : Int");
    assert!(super::parse_binding_list(&mut p).is_err());
}
