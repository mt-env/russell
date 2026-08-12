use crate::frontend::lexer::lex;
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::*;

fn parser_from(input: &str) -> Parser<'_> {
    Parser::new(lex(input).expect("test input should lex successfully"))
}

fn parse(input: &str) -> ParsedDefn<'_> {
    let mut p = parser_from(input);
    super::parse_defn(&mut p).unwrap()
}

fn ty(offset: usize, node: Type<'_>) -> ParsedType<'_> {
    ParsedType::new(offset, node)
}

fn arm<'a>(
    offset: usize,
    id: &'a str,
    bindings: Vec<ParsedBinding<'a>>,
) -> ParsedConstructorArm<'a> {
    ParsedConstructorArm::new(offset, id, bindings)
}

// ─── fn definitions ─────────────────────────────────────────────────

#[test]
fn fn_no_params() {
    assert_eq!(
        parse("fn main() -> Int { return 0; }"),
        ParsedDefn::make_fn(
            0,
            "main".into(),
            vec![],
            ty(13, Type::Int),
            vec![ParsedStmt::make_return(
                19,
                ParsedExpr::new(26, ExprKind::Int(0))
            )]
        )
    );
}

#[test]
fn fn_one_param() {
    assert_eq!(
        parse("fn id(x: Int) -> Int { return x; }"),
        ParsedDefn::make_fn(
            0,
            "id".into(),
            vec![ParsedBinding::new(6, "x".into(), ty(9, Type::Int))],
            ty(17, Type::Int),
            vec![ParsedStmt::make_return(
                23,
                ParsedExpr::new(30, ExprKind::Id("x".into()))
            )]
        )
    );
}

#[test]
fn fn_two_params() {
    assert_eq!(
        parse("fn add(a: Int, b: Int) -> Int { return a + b; }"),
        ParsedDefn::make_fn(
            0,
            "add".into(),
            vec![
                ParsedBinding::new(7, "a".into(), ty(10, Type::Int)),
                ParsedBinding::new(15, "b".into(), ty(18, Type::Int)),
            ],
            ty(26, Type::Int),
            vec![ParsedStmt::make_return(
                32,
                ParsedExpr::new(
                    39,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(39, ExprKind::Id("a".into()))),
                        Box::new(ParsedExpr::new(43, ExprKind::Id("b".into())))
                    )
                )
            )]
        )
    );
}

#[test]
fn fn_multiple_statements() {
    assert_eq!(
        parse("fn foo() -> Int { let x = 1; let y = 2; return x + y; }"),
        ParsedDefn::make_fn(
            0,
            "foo".into(),
            vec![],
            ty(12, Type::Int),
            vec![
                ParsedStmt::make_let(18, "x".into(), ParsedExpr::new(26, ExprKind::Int(1))),
                ParsedStmt::make_let(29, "y".into(), ParsedExpr::new(37, ExprKind::Int(2))),
                ParsedStmt::make_return(
                    40,
                    ParsedExpr::new(
                        47,
                        ExprKind::Plus(
                            Box::new(ParsedExpr::new(47, ExprKind::Id("x".into()))),
                            Box::new(ParsedExpr::new(51, ExprKind::Id("y".into())))
                        )
                    )
                ),
            ]
        )
    );
}

#[test]
fn fn_with_float_return() {
    assert_eq!(
        parse("fn pi() -> Float { return 3.14; }"),
        ParsedDefn::make_fn(
            0,
            "pi".into(),
            vec![],
            ty(11, Type::Float),
            vec![ParsedStmt::make_return(
                19,
                ParsedExpr::new(26, ExprKind::Float(3.14))
            )]
        )
    );
}

#[test]
fn fn_with_bool_return() {
    assert_eq!(
        parse("fn yes() -> Bool { return true; }"),
        ParsedDefn::make_fn(
            0,
            "yes".into(),
            vec![],
            ty(12, Type::Bool),
            vec![ParsedStmt::make_return(
                19,
                ParsedExpr::new(26, ExprKind::Bool(true))
            )]
        )
    );
}

#[test]
fn fn_with_fn_type_param() {
    assert_eq!(
        parse("fn apply(f: Int -> Int, x: Int) -> Int { return f(x); }"),
        ParsedDefn::make_fn(
            0,
            "apply".into(),
            vec![
                ParsedBinding::new(
                    9,
                    "f".into(),
                    ty(
                        12,
                        Type::Fn(Box::new(ty(12, Type::Int)), Box::new(ty(19, Type::Int)))
                    )
                ),
                ParsedBinding::new(24, "x".into(), ty(27, Type::Int)),
            ],
            ty(35, Type::Int),
            vec![ParsedStmt::make_return(
                41,
                ParsedExpr::new(
                    48,
                    ExprKind::Call(
                        Box::new(ParsedExpr::new(48, ExprKind::Id("f".into()))),
                        vec![ParsedExpr::new(50, ExprKind::Id("x".into()))]
                    )
                )
            )]
        )
    );
}

#[test]
fn fn_empty_body() {
    // A function with no statements (unusual but parseable)
    assert_eq!(
        parse("fn noop() -> Int { }"),
        ParsedDefn::make_fn(0, "noop".into(), vec![], ty(13, Type::Int), vec![])
    );
}

#[test]
fn fn_with_read_and_echo() {
    assert_eq!(
        parse("fn main() -> Int { read Int x; echo Int x; return 0; }"),
        ParsedDefn::make_fn(
            0,
            "main".into(),
            vec![],
            ty(13, Type::Int),
            vec![
                ParsedStmt::make_read(19, ty(24, Type::Int), "x".into()),
                ParsedStmt::make_echo(
                    31,
                    ty(36, Type::Int),
                    ParsedExpr::new(40, ExprKind::Id("x".into()))
                ),
                ParsedStmt::make_return(43, ParsedExpr::new(50, ExprKind::Int(0))),
            ]
        )
    );
}

// ─── typedef ────────────────────────────────────────────────────────

#[test]
fn typedef_empty() {
    assert_eq!(
        parse("typedef Empty { }"),
        ParsedDefn::make_typedef(0, "Empty".into(), vec![], vec![])
    );
}

#[test]
fn typedef_single_nullary_constructor() {
    assert_eq!(
        parse("typedef Unit { unit() }"),
        ParsedDefn::make_typedef(0, "Unit".into(), vec![], vec![arm(15, "unit", vec![])])
    );
}

#[test]
fn typedef_single_constructor_with_field() {
    assert_eq!(
        parse("typedef Wrapper { wrap(x: Int) }"),
        ParsedDefn::make_typedef(
            0,
            "Wrapper".into(),
            vec![],
            vec![arm(
                18,
                "wrap",
                vec![ParsedBinding::new(23, "x".into(), ty(26, Type::Int))]
            )]
        )
    );
}

#[test]
fn typedef_multiple_constructors() {
    assert_eq!(
        parse("typedef Color { red(), green(), blue() }"),
        ParsedDefn::make_typedef(
            0,
            "Color".into(),
            vec![],
            vec![
                arm(16, "red", vec![]),
                arm(23, "green", vec![]),
                arm(32, "blue", vec![]),
            ]
        )
    );
}

#[test]
fn typedef_constructors_with_fields() {
    assert_eq!(
        parse("typedef Shape { circle(r: Float), rect(w: Float, h: Float) }"),
        ParsedDefn::make_typedef(
            0,
            "Shape".into(),
            vec![],
            vec![
                arm(
                    16,
                    "circle",
                    vec![ParsedBinding::new(23, "r".into(), ty(26, Type::Float))]
                ),
                arm(
                    34,
                    "rect",
                    vec![
                        ParsedBinding::new(39, "w".into(), ty(42, Type::Float)),
                        ParsedBinding::new(49, "h".into(), ty(52, Type::Float)),
                    ]
                ),
            ]
        )
    );
}

#[test]
fn typedef_option_pattern() {
    assert_eq!(
        parse("typedef Option { some(x: Int), none() }"),
        ParsedDefn::make_typedef(
            0,
            "Option".into(),
            vec![],
            vec![
                arm(
                    17,
                    "some",
                    vec![ParsedBinding::new(22, "x".into(), ty(25, Type::Int))]
                ),
                arm(31, "none", vec![]),
            ]
        )
    );
}

#[test]
fn typedef_option_type_params() {
    assert_eq!(
        parse("typedef Option('a) { some(x: 'a), none() }"),
        ParsedDefn::make_typedef(
            0,
            "Option".into(),
            vec!["'a"],
            vec![
                arm(
                    21,
                    "some",
                    vec![ParsedBinding::new(
                        26,
                        "x".into(),
                        ty(29, Type::TypeParam("'a".into()))
                    )]
                ),
                arm(34, "none", vec![]),
            ]
        )
    );
}

#[test]
fn typedef_result_type_params() {
    assert_eq!(
        parse("typedef Result('ok, 'err) { ok(good: 'ok), err(bad: 'err) }"),
        ParsedDefn::make_typedef(
            0,
            "Result".into(),
            vec!["'ok", "'err"],
            vec![
                arm(
                    28,
                    "ok",
                    vec![ParsedBinding::new(
                        31,
                        "good".into(),
                        ty(37, Type::TypeParam("'ok".into()))
                    )]
                ),
                arm(
                    43,
                    "err",
                    vec![ParsedBinding::new(
                        47,
                        "bad".into(),
                        ty(52, Type::TypeParam("'err".into()))
                    )]
                )
            ]
        )
    )
}

// ─── dispatch errors ────────────────────────────────────────────────

#[test]
fn error_on_int_literal() {
    let mut p = parser_from("42");
    assert!(super::parse_defn(&mut p).is_err());
}

#[test]
fn error_on_let_keyword() {
    let mut p = parser_from("let x = 1;");
    assert!(super::parse_defn(&mut p).is_err());
}

#[test]
fn error_on_return_keyword() {
    let mut p = parser_from("return 0;");
    assert!(super::parse_defn(&mut p).is_err());
}
