use crate::frontend::lexer::lex;
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::{ParsedBinding, ParsedType, Type};

fn parser_from(input: &str) -> Parser<'_> {
    Parser::new(lex(input).expect("test input should lex successfully"))
}

fn parse(input: &str) -> ParsedType<'_> {
    let mut p = parser_from(input);
    super::parse_type(&mut p).unwrap()
}

fn ty(offset: usize, node: Type<'_>) -> ParsedType<'_> {
    ParsedType::new(offset, node)
}

// ─── parse_type ─────────────────────────────────────────────────────

#[test]
fn int_type() {
    assert_eq!(parse("Int"), ty(0, Type::Int));
}

#[test]
fn float_type() {
    assert_eq!(parse("Float"), ty(0, Type::Float));
}

#[test]
fn bool_type() {
    assert_eq!(parse("Bool"), ty(0, Type::Bool));
}

#[test]
fn typeid() {
    assert_eq!(parse("MyType"), ty(0, Type::TypeId("MyType".into())));
}

#[test]
fn typeapp_error_no_args() {
    let mut p = parser_from("MyType()");
    assert!(super::parse_type(&mut p).is_err());
}

#[test]
fn typeapp_single_arg() {
    assert_eq!(
        parse("List(Int)"),
        ty(0, Type::TypeApp("List".into(), vec![ty(5, Type::Int)]))
    );
}

#[test]
fn typeapp_multiple_args() {
    assert_eq!(
        parse("Result(Int, Bool)"),
        ty(
            0,
            Type::TypeApp("Result".into(), vec![ty(7, Type::Int), ty(12, Type::Bool)])
        )
    );
}

#[test]
fn typeapp_nested_args() {
    assert_eq!(
        parse("Map(Int, List(Bool))"),
        ty(
            0,
            Type::TypeApp(
                "Map".into(),
                vec![
                    ty(4, Type::Int),
                    ty(9, Type::TypeApp("List".into(), vec![ty(14, Type::Bool)])),
                ]
            )
        )
    );
}

#[test]
fn typeparam() {
    assert_eq!(parse("'abc"), ty(0, Type::TypeParam("'abc".into())));
}

#[test]
fn fn_type_with_typeparams() {
    assert_eq!(
        parse("'a -> 'b"),
        ty(
            0,
            Type::Fn(
                Box::new(ty(0, Type::TypeParam("'a".into()))),
                Box::new(ty(6, Type::TypeParam("'b".into())))
            )
        )
    );
}

#[test]
fn fn_type_simple() {
    assert_eq!(
        parse("Int -> Float"),
        ty(
            0,
            Type::Fn(Box::new(ty(0, Type::Int)), Box::new(ty(7, Type::Float)))
        )
    );
}

#[test]
fn fn_type_right_associative() {
    // Int -> Float -> Bool = Int -> (Float -> Bool)
    assert_eq!(
        parse("Int -> Float -> Bool"),
        ty(
            0,
            Type::Fn(
                Box::new(ty(0, Type::Int)),
                Box::new(ty(
                    7,
                    Type::Fn(Box::new(ty(7, Type::Float)), Box::new(ty(16, Type::Bool)))
                ))
            )
        )
    );
}

#[test]
fn fn_type_three_deep() {
    // Int -> Float -> Bool -> Int = Int -> (Float -> (Bool -> Int))
    assert_eq!(
        parse("Int -> Float -> Bool -> Int"),
        ty(
            0,
            Type::Fn(
                Box::new(ty(0, Type::Int)),
                Box::new(ty(
                    7,
                    Type::Fn(
                        Box::new(ty(7, Type::Float)),
                        Box::new(ty(
                            16,
                            Type::Fn(Box::new(ty(16, Type::Bool)), Box::new(ty(24, Type::Int)))
                        ))
                    )
                ))
            )
        )
    );
}

#[test]
fn fn_type_with_typeid() {
    assert_eq!(
        parse("MyType -> Int"),
        ty(
            0,
            Type::Fn(
                Box::new(ty(0, Type::TypeId("MyType".into()))),
                Box::new(ty(10, Type::Int))
            )
        )
    );
}

#[test]
fn fn_type_with_typeapp() {
    assert_eq!(
        parse("List(Int) -> Result(Int, Bool)"),
        ty(
            0,
            Type::Fn(
                Box::new(ty(0, Type::TypeApp("List".into(), vec![ty(5, Type::Int)]))),
                Box::new(ty(
                    13,
                    Type::TypeApp("Result".into(), vec![ty(20, Type::Int), ty(25, Type::Bool)])
                )),
            )
        )
    );
}

#[test]
fn parenthesized_type() {
    assert_eq!(parse("(Int)"), ty(1, Type::Int));
}

#[test]
fn parenthesized_fn_type_grouping() {
    assert_eq!(
        parse("(Int -> Bool) -> Int"),
        ty(
            0,
            Type::Fn(
                Box::new(ty(
                    1,
                    Type::Fn(Box::new(ty(1, Type::Int)), Box::new(ty(8, Type::Bool)))
                )),
                Box::new(ty(17, Type::Int))
            )
        )
    );
}

#[test]
fn nested_parentheses() {
    assert_eq!(
        parse("Int -> (Bool -> (Int -> Bool) -> Int) -> Bool"),
        ty(
            0,
            Type::Fn(
                Box::new(ty(0, Type::Int)),
                Box::new(ty(
                    7,
                    Type::Fn(
                        Box::new(ty(
                            8,
                            Type::Fn(
                                Box::new(ty(8, Type::Bool)),
                                Box::new(ty(
                                    16,
                                    Type::Fn(
                                        Box::new(ty(
                                            17,
                                            Type::Fn(
                                                Box::new(ty(17, Type::Int)),
                                                Box::new(ty(24, Type::Bool))
                                            )
                                        )),
                                        Box::new(ty(33, Type::Int))
                                    )
                                ))
                            )
                        )),
                        Box::new(ty(41, Type::Bool))
                    )
                ))
            )
        )
    )
}

#[test]
fn parenthesized_type_error_missing_rparen() {
    let mut p = parser_from("(Int -> Bool");
    assert!(super::parse_type(&mut p).is_err());
}

#[test]
fn typeapp_error_missing_rparen() {
    let mut p = parser_from("List(Int");
    assert!(super::parse_type(&mut p).is_err());
}

#[test]
fn typeapp_error_trailing_comma() {
    let mut p = parser_from("Result(Int,)");
    assert!(super::parse_type(&mut p).is_err());
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
    assert_eq!(b, ParsedBinding::new(0, "x".into(), ty(4, Type::Int)));
}

#[test]
fn binding_with_float_type() {
    let mut p = parser_from("val : Float");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(b, ParsedBinding::new(0, "val".into(), ty(6, Type::Float)));
}

#[test]
fn binding_with_bool_type() {
    let mut p = parser_from("flag : Bool");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(b, ParsedBinding::new(0, "flag".into(), ty(7, Type::Bool)));
}

#[test]
fn binding_with_fn_type() {
    let mut p = parser_from("f : Int -> Bool");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(
        b,
        ParsedBinding::new(
            0,
            "f".into(),
            ty(
                4,
                Type::Fn(Box::new(ty(4, Type::Int)), Box::new(ty(11, Type::Bool)))
            )
        )
    );
}

#[test]
fn binding_with_typeid() {
    let mut p = parser_from("x : MyType");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(
        b,
        ParsedBinding::new(0, "x".into(), ty(4, Type::TypeId("MyType".into())))
    );
}

#[test]
fn binding_with_typeapp() {
    let mut p = parser_from("xs : List(Int)");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(
        b,
        ParsedBinding::new(
            0,
            "xs".into(),
            ty(5, Type::TypeApp("List".into(), vec![ty(10, Type::Int)]))
        )
    );
}

#[test]
fn binding_error_typeapp_no_args() {
    let mut p = parser_from("xs : List()");
    assert!(super::parse_binding(&mut p).is_err());
}

#[test]
fn binding_with_typeparam() {
    let mut p = parser_from("x : 'a");
    let b = super::parse_binding(&mut p).unwrap();
    assert_eq!(
        b,
        ParsedBinding::new(0, "x".into(), ty(4, Type::TypeParam("'a".into())))
    );
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
    assert_eq!(
        bindings,
        vec![ParsedBinding::new(1, "x".into(), ty(5, Type::Int))]
    );
}

#[test]
fn multiple_bindings() {
    let mut p = parser_from("(x : Int, y : Float)");
    let bindings = super::parse_binding_list(&mut p).unwrap();
    assert_eq!(
        bindings,
        vec![
            ParsedBinding::new(1, "x".into(), ty(5, Type::Int)),
            ParsedBinding::new(10, "y".into(), ty(14, Type::Float)),
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
            ParsedBinding::new(1, "a".into(), ty(5, Type::Int)),
            ParsedBinding::new(10, "b".into(), ty(14, Type::Float)),
            ParsedBinding::new(21, "c".into(), ty(25, Type::Bool)),
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
            ParsedBinding::new(
                1,
                "f".into(),
                ty(
                    5,
                    Type::Fn(Box::new(ty(5, Type::Int)), Box::new(ty(12, Type::Bool)))
                )
            ),
            ParsedBinding::new(18, "x".into(), ty(22, Type::Int)),
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
