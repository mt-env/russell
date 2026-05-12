use crate::frontend::lexer::lex;
use crate::frontend::parser::ast::*;
use crate::frontend::parser::Parser;

fn parser_from(input: &str) -> Parser {
    Parser::new(lex(input))
}

fn parse(input: &str) -> ParsedDefn {
    let mut p = parser_from(input);
    super::parse_defn(&mut p).unwrap()
}

// ─── fn definitions ─────────────────────────────────────────────────

#[test]
fn fn_no_params() {
    assert_eq!(
        parse("fn main() -> Int { return 0; }"),
        Defn::Fn(
            "main".into(),
            vec![],
            Type::Int,
            vec![Stmt::Return(Expr::parsed(ExprKind::Int(0)))]
        )
    );
}

#[test]
fn fn_one_param() {
    assert_eq!(
        parse("fn id(x: Int) -> Int { return x; }"),
        Defn::Fn(
            "id".into(),
            vec![Binding::new("x".into(), Type::Int)],
            Type::Int,
            vec![Stmt::Return(Expr::parsed(ExprKind::Id("x".into())))]
        )
    );
}

#[test]
fn fn_two_params() {
    assert_eq!(
        parse("fn add(a: Int, b: Int) -> Int { return a + b; }"),
        Defn::Fn(
            "add".into(),
            vec![
                Binding::new("a".into(), Type::Int),
                Binding::new("b".into(), Type::Int),
            ],
            Type::Int,
            vec![Stmt::Return(Expr::parsed(ExprKind::Plus(
                Box::new(Expr::parsed(ExprKind::Id("a".into()))),
                Box::new(Expr::parsed(ExprKind::Id("b".into())))
            )))]
        )
    );
}

#[test]
fn fn_multiple_statements() {
    assert_eq!(
        parse("fn foo() -> Int { let x = 1; let y = 2; return x + y; }"),
        Defn::Fn(
            "foo".into(),
            vec![],
            Type::Int,
            vec![
                Stmt::Let("x".into(), Expr::parsed(ExprKind::Int(1))),
                Stmt::Let("y".into(), Expr::parsed(ExprKind::Int(2))),
                Stmt::Return(Expr::parsed(ExprKind::Plus(
                    Box::new(Expr::parsed(ExprKind::Id("x".into()))),
                    Box::new(Expr::parsed(ExprKind::Id("y".into())))
                ))),
            ]
        )
    );
}

#[test]
fn fn_with_float_return() {
    assert_eq!(
        parse("fn pi() -> Float { return 3.14; }"),
        Defn::Fn(
            "pi".into(),
            vec![],
            Type::Float,
            vec![Stmt::Return(Expr::parsed(ExprKind::Float(3.14)))]
        )
    );
}

#[test]
fn fn_with_bool_return() {
    assert_eq!(
        parse("fn yes() -> Bool { return true; }"),
        Defn::Fn(
            "yes".into(),
            vec![],
            Type::Bool,
            vec![Stmt::Return(Expr::parsed(ExprKind::Bool(true)))]
        )
    );
}

#[test]
fn fn_with_fn_type_param() {
    assert_eq!(
        parse("fn apply(f: Int -> Int, x: Int) -> Int { return f(x); }"),
        Defn::Fn(
            "apply".into(),
            vec![
                Binding::new(
                    "f".into(),
                    Type::Fn(Box::new(Type::Int), Box::new(Type::Int))
                ),
                Binding::new("x".into(), Type::Int),
            ],
            Type::Int,
            vec![Stmt::Return(Expr::parsed(ExprKind::Call(
                Box::new(Expr::parsed(ExprKind::Id("f".into()))),
                vec![Expr::parsed(ExprKind::Id("x".into()))]
            )))]
        )
    );
}

#[test]
fn fn_empty_body() {
    // A function with no statements (unusual but parseable)
    assert_eq!(
        parse("fn noop() -> Int { }"),
        Defn::Fn("noop".into(), vec![], Type::Int, vec![])
    );
}

#[test]
fn fn_with_read_and_echo() {
    assert_eq!(
        parse("fn main() -> Int { read Int x; echo Int x; return 0; }"),
        Defn::Fn(
            "main".into(),
            vec![],
            Type::Int,
            vec![
                Stmt::Read(Type::Int, "x".into()),
                Stmt::Echo(Type::Int, Expr::parsed(ExprKind::Id("x".into()))),
                Stmt::Return(Expr::parsed(ExprKind::Int(0))),
            ]
        )
    );
}

// ─── typedef ────────────────────────────────────────────────────────

#[test]
fn typedef_empty() {
    assert_eq!(
        parse("typedef Empty { }"),
        Defn::Typedef("Empty".into(), vec![])
    );
}

#[test]
fn typedef_single_nullary_constructor() {
    assert_eq!(
        parse("typedef Unit { unit() }"),
        Defn::Typedef("Unit".into(), vec![("unit".into(), vec![])])
    );
}

#[test]
fn typedef_single_constructor_with_field() {
    assert_eq!(
        parse("typedef Wrapper { wrap(x: Int) }"),
        Defn::Typedef(
            "Wrapper".into(),
            vec![("wrap".into(), vec![Binding::new("x".into(), Type::Int)])]
        )
    );
}

#[test]
fn typedef_multiple_constructors() {
    assert_eq!(
        parse("typedef Color { red(), green(), blue() }"),
        Defn::Typedef(
            "Color".into(),
            vec![
                ("red".into(), vec![]),
                ("green".into(), vec![]),
                ("blue".into(), vec![]),
            ]
        )
    );
}

#[test]
fn typedef_constructors_with_fields() {
    assert_eq!(
        parse("typedef Shape { circle(r: Float), rect(w: Float, h: Float) }"),
        Defn::Typedef(
            "Shape".into(),
            vec![
                (
                    "circle".into(),
                    vec![Binding::new("r".into(), Type::Float)]
                ),
                (
                    "rect".into(),
                    vec![
                        Binding::new("w".into(), Type::Float),
                        Binding::new("h".into(), Type::Float),
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
        Defn::Typedef(
            "Option".into(),
            vec![
                ("some".into(), vec![Binding::new("x".into(), Type::Int)]),
                ("none".into(), vec![]),
            ]
        )
    );
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
