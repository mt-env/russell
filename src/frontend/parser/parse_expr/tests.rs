use crate::frontend::lexer::lex;
use crate::frontend::parser::ast::{Binding, Expr, ExprKind, ParsedExpr, Type};
use crate::frontend::parser::Parser;

fn parser_from(input: &str) -> Parser {
    Parser::new(lex(input))
}

fn parse(input: &str) -> ParsedExpr {
    let mut p = parser_from(input);
    super::parse_expr(&mut p).unwrap()
}

// ─── atoms ──────────────────────────────────────────────────────────

#[test]
fn int_literal() {
    assert_eq!(parse("42"), Expr::parsed(ExprKind::Int(42)));
}

#[test]
fn zero() {
    assert_eq!(parse("0"), Expr::parsed(ExprKind::Int(0)));
}

#[test]
fn float_literal() {
    assert_eq!(parse("3.14"), Expr::parsed(ExprKind::Float(3.14)));
}

#[test]
fn true_literal() {
    assert_eq!(parse("true"), Expr::parsed(ExprKind::Bool(true)));
}

#[test]
fn false_literal() {
    assert_eq!(parse("false"), Expr::parsed(ExprKind::Bool(false)));
}

#[test]
fn identifier() {
    assert_eq!(parse("foo"), Expr::parsed(ExprKind::Id("foo".into())));
}

// ─── unary operators ────────────────────────────────────────────────

#[test]
fn negate_int() {
    assert_eq!(parse("-1"), Expr::parsed(ExprKind::Neg(Box::new(Expr::parsed(ExprKind::Int(1))))));
}

#[test]
fn negate_identifier() {
    assert_eq!(parse("-x"), Expr::parsed(ExprKind::Neg(Box::new(Expr::parsed(ExprKind::Id("x".into()))))));
}

#[test]
fn bang_bool() {
    assert_eq!(parse("!true"), Expr::parsed(ExprKind::Bang(Box::new(Expr::parsed(ExprKind::Bool(true))))));
}

#[test]
fn bang_identifier() {
    assert_eq!(parse("!x"), Expr::parsed(ExprKind::Bang(Box::new(Expr::parsed(ExprKind::Id("x".into()))))));
}

#[test]
fn double_negate() {
    assert_eq!(parse("--1"), Expr::parsed(ExprKind::Neg(Box::new(Expr::parsed(ExprKind::Neg(Box::new(Expr::parsed(ExprKind::Int(1)))))))));
}

#[test]
fn double_bang() {
    assert_eq!(
        parse("!!true"),
        Expr::parsed(ExprKind::Bang(Box::new(Expr::parsed(ExprKind::Bang(Box::new(Expr::parsed(ExprKind::Bool(true))))))))
    );
}

// ─── binary operators ───────────────────────────────────────────────

#[test]
fn addition() {
    assert_eq!(
        parse("1 + 2"),
        Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Int(2)))))
    );
}

#[test]
fn subtraction() {
    assert_eq!(
        parse("3 - 1"),
        Expr::parsed(ExprKind::Minus(Box::new(Expr::parsed(ExprKind::Int(3))), Box::new(Expr::parsed(ExprKind::Int(1)))))
    );
}

#[test]
fn multiplication() {
    assert_eq!(
        parse("2 * 3"),
        Expr::parsed(ExprKind::Mult(Box::new(Expr::parsed(ExprKind::Int(2))), Box::new(Expr::parsed(ExprKind::Int(3)))))
    );
}

#[test]
fn division() {
    assert_eq!(
        parse("6 / 2"),
        Expr::parsed(ExprKind::Div(Box::new(Expr::parsed(ExprKind::Int(6))), Box::new(Expr::parsed(ExprKind::Int(2)))))
    );
}

#[test]
fn less_than() {
    assert_eq!(
        parse("a < b"),
        Expr::parsed(ExprKind::Less(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))
    );
}

#[test]
fn less_than_or_eq() {
    assert_eq!(
        parse("a <= b"),
        Expr::parsed(ExprKind::LessEq(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))
    );
}

#[test]
fn greater_than() {
    assert_eq!(
        parse("a > b"),
        Expr::parsed(ExprKind::Greater(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))
    );
}

#[test]
fn greater_than_or_eq() {
    assert_eq!(
        parse("a >= b"),
        Expr::parsed(ExprKind::GreaterEq(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))
    );
}

#[test]
fn equality() {
    assert_eq!(
        parse("a == b"),
        Expr::parsed(ExprKind::Eq(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))
    );
}

#[test]
fn not_equal() {
    assert_eq!(
        parse("a != b"),
        Expr::parsed(ExprKind::NotEq(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))
    );
}

#[test]
fn logical_or() {
    assert_eq!(
        parse("a || b"),
        Expr::parsed(ExprKind::Or(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))
    );
}

#[test]
fn logical_and() {
    assert_eq!(
        parse("a && b"),
        Expr::parsed(ExprKind::And(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))
    );
}

#[test]
fn pipe() {
    assert_eq!(
        parse("a |> f"),
        Expr::parsed(ExprKind::Pipe(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("f".into())))))
    );
}

// ─── precedence ─────────────────────────────────────────────────────

#[test]
fn mult_before_add() {
    // 1 + 2 * 3 = 1 + (2 * 3)
    assert_eq!(
        parse("1 + 2 * 3"),
        Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Mult(Box::new(Expr::parsed(ExprKind::Int(2))), Box::new(Expr::parsed(ExprKind::Int(3))))))))
    );
}

#[test]
fn mult_before_sub() {
    // 1 - 2 * 3 = 1 - (2 * 3)
    assert_eq!(
        parse("1 - 2 * 3"),
        Expr::parsed(ExprKind::Minus(Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Mult(Box::new(Expr::parsed(ExprKind::Int(2))), Box::new(Expr::parsed(ExprKind::Int(3))))))))
    );
}

#[test]
fn add_before_relational() {
    // a + b < c = (a + b) < c
    assert_eq!(
        parse("a + b < c"),
        Expr::parsed(ExprKind::Less(
            Box::new(Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("c".into())))
        ))
    );
}

#[test]
fn relational_before_equality() {
    // a < b == c = (a < b) == c
    assert_eq!(
        parse("a < b == c"),
        Expr::parsed(ExprKind::Eq(
            Box::new(Expr::parsed(ExprKind::Less(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("c".into())))
        ))
    );
}

#[test]
fn equality_before_and() {
    // a == b && c = (a == b) && c
    assert_eq!(
        parse("a == b && c"),
        Expr::parsed(ExprKind::And(
            Box::new(Expr::parsed(ExprKind::Eq(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("c".into())))
        ))
    );
}

#[test]
fn and_before_or() {
    // a && b || c = (a && b) || c
    assert_eq!(
        parse("a && b || c"),
        Expr::parsed(ExprKind::Or(
            Box::new(Expr::parsed(ExprKind::And(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("c".into())))
        ))
    );
}

#[test]
fn or_before_pipe() {
    // a || b |> f = (a || b) |> f
    assert_eq!(
        parse("a || b |> f"),
        Expr::parsed(ExprKind::Pipe(
            Box::new(Expr::parsed(ExprKind::Or(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("f".into())))
        ))
    );
}

#[test]
fn unary_binds_tighter_than_mult() {
    // -1 * 2 = (-1) * 2
    assert_eq!(
        parse("-1 * 2"),
        Expr::parsed(ExprKind::Mult(Box::new(Expr::parsed(ExprKind::Neg(Box::new(Expr::parsed(ExprKind::Int(1)))))), Box::new(Expr::parsed(ExprKind::Int(2)))))
    );
}

#[test]
fn unary_binds_tighter_than_add() {
    // -1 + 2 = (-1) + 2
    assert_eq!(
        parse("-1 + 2"),
        Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Neg(Box::new(Expr::parsed(ExprKind::Int(1)))))), Box::new(Expr::parsed(ExprKind::Int(2)))))
    );
}

#[test]
fn bang_binds_tighter_than_and() {
    // !a && b = (!a) && b
    assert_eq!(
        parse("!a && b"),
        Expr::parsed(ExprKind::And(
            Box::new(Expr::parsed(ExprKind::Bang(Box::new(Expr::parsed(ExprKind::Id("a".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("b".into())))
        ))
    );
}

// ─── associativity (left-to-right) ──────────────────────────────────

#[test]
fn addition_left_assoc() {
    // 1 + 2 + 3 = (1 + 2) + 3
    assert_eq!(
        parse("1 + 2 + 3"),
        Expr::parsed(ExprKind::Plus(
            Box::new(Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Int(2)))))),
            Box::new(Expr::parsed(ExprKind::Int(3)))
        ))
    );
}

#[test]
fn subtraction_left_assoc() {
    // 5 - 3 - 1 = (5 - 3) - 1
    assert_eq!(
        parse("5 - 3 - 1"),
        Expr::parsed(ExprKind::Minus(
            Box::new(Expr::parsed(ExprKind::Minus(Box::new(Expr::parsed(ExprKind::Int(5))), Box::new(Expr::parsed(ExprKind::Int(3)))))),
            Box::new(Expr::parsed(ExprKind::Int(1)))
        ))
    );
}

#[test]
fn multiplication_left_assoc() {
    // 2 * 3 * 4 = (2 * 3) * 4
    assert_eq!(
        parse("2 * 3 * 4"),
        Expr::parsed(ExprKind::Mult(
            Box::new(Expr::parsed(ExprKind::Mult(Box::new(Expr::parsed(ExprKind::Int(2))), Box::new(Expr::parsed(ExprKind::Int(3)))))),
            Box::new(Expr::parsed(ExprKind::Int(4)))
        ))
    );
}

#[test]
fn pipe_left_assoc() {
    // x |> f |> g = (x |> f) |> g
    assert_eq!(
        parse("x |> f |> g"),
        Expr::parsed(ExprKind::Pipe(
            Box::new(Expr::parsed(ExprKind::Pipe(Box::new(Expr::parsed(ExprKind::Id("x".into()))), Box::new(Expr::parsed(ExprKind::Id("f".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("g".into())))
        ))
    );
}

#[test]
fn mixed_add_sub_left_assoc() {
    // 1 + 2 - 3 = (1 + 2) - 3
    assert_eq!(
        parse("1 + 2 - 3"),
        Expr::parsed(ExprKind::Minus(
            Box::new(Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Int(2)))))),
            Box::new(Expr::parsed(ExprKind::Int(3)))
        ))
    );
}

// ─── parenthesized expressions ──────────────────────────────────────

#[test]
fn parens_identity() {
    assert_eq!(parse("(42)"), Expr::parsed(ExprKind::Int(42)));
}

#[test]
fn parens_override_precedence() {
    // (1 + 2) * 3
    assert_eq!(
        parse("(1 + 2) * 3"),
        Expr::parsed(ExprKind::Mult(
            Box::new(Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Int(2)))))),
            Box::new(Expr::parsed(ExprKind::Int(3)))
        ))
    );
}

#[test]
fn nested_parens() {
    assert_eq!(parse("((1))"), Expr::parsed(ExprKind::Int(1)));
}

#[test]
fn parens_in_right_operand() {
    // 2 * (3 + 4)
    assert_eq!(
        parse("2 * (3 + 4)"),
        Expr::parsed(ExprKind::Mult(
            Box::new(Expr::parsed(ExprKind::Int(2))),
            Box::new(Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Int(3))), Box::new(Expr::parsed(ExprKind::Int(4))))))
        ))
    );
}

// ─── function calls ─────────────────────────────────────────────────

#[test]
fn call_no_args() {
    assert_eq!(
        parse("f()"),
        Expr::parsed(ExprKind::Call(Box::new(Expr::parsed(ExprKind::Id("f".into()))), vec![]))
    );
}

#[test]
fn call_one_arg() {
    assert_eq!(
        parse("f(1)"),
        Expr::parsed(ExprKind::Call(Box::new(Expr::parsed(ExprKind::Id("f".into()))), vec![Expr::parsed(ExprKind::Int(1))]))
    );
}

#[test]
fn call_multiple_args() {
    assert_eq!(
        parse("f(1, 2, 3)"),
        Expr::parsed(ExprKind::Call(
            Box::new(Expr::parsed(ExprKind::Id("f".into()))),
            vec![Expr::parsed(ExprKind::Int(1)), Expr::parsed(ExprKind::Int(2)), Expr::parsed(ExprKind::Int(3))]
        ))
    );
}

#[test]
fn call_with_expr_arg() {
    assert_eq!(
        parse("f(1 + 2)"),
        Expr::parsed(ExprKind::Call(
            Box::new(Expr::parsed(ExprKind::Id("f".into()))),
            vec![Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Int(2)))))]
        ))
    );
}

#[test]
fn chained_calls() {
    // f(x)(y) = Call(Call(f, [x]), [y])
    assert_eq!(
        parse("f(x)(y)"),
        Expr::parsed(ExprKind::Call(
            Box::new(Expr::parsed(ExprKind::Call(
                Box::new(Expr::parsed(ExprKind::Id("f".into()))),
                vec![Expr::parsed(ExprKind::Id("x".into()))]
            ))),
            vec![Expr::parsed(ExprKind::Id("y".into()))]
        ))
    );
}

#[test]
fn call_in_binary_expr() {
    // f(x) + 1
    assert_eq!(
        parse("f(x) + 1"),
        Expr::parsed(ExprKind::Plus(
            Box::new(Expr::parsed(ExprKind::Call(
                Box::new(Expr::parsed(ExprKind::Id("f".into()))),
                vec![Expr::parsed(ExprKind::Id("x".into()))]
            ))),
            Box::new(Expr::parsed(ExprKind::Int(1)))
        ))
    );
}

#[test]
fn binary_expr_then_call() {
    // 1 + f(x)
    assert_eq!(
        parse("1 + f(x)"),
        Expr::parsed(ExprKind::Plus(
            Box::new(Expr::parsed(ExprKind::Int(1))),
            Box::new(Expr::parsed(ExprKind::Call(
                Box::new(Expr::parsed(ExprKind::Id("f".into()))),
                vec![Expr::parsed(ExprKind::Id("x".into()))]
            )))
        ))
    );
}

#[test]
fn negate_call() {
    // -f(x) = Neg(Call(f, [x]))
    assert_eq!(
        parse("-f(x)"),
        Expr::parsed(ExprKind::Neg(Box::new(Expr::parsed(ExprKind::Call(
            Box::new(Expr::parsed(ExprKind::Id("f".into()))),
            vec![Expr::parsed(ExprKind::Id("x".into()))]
        )))))
    );
}

// ─── if-then-else ───────────────────────────────────────────────────

#[test]
fn simple_if() {
    assert_eq!(
        parse("if true then 1 else 2"),
        Expr::parsed(ExprKind::If(Box::new(Expr::parsed(ExprKind::Bool(true))), Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Int(2)))))
    );
}

#[test]
fn if_with_condition_expr() {
    assert_eq!(
        parse("if a == b then a else b"),
        Expr::parsed(ExprKind::If(
            Box::new(Expr::parsed(ExprKind::Eq(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("a".into()))),
            Box::new(Expr::parsed(ExprKind::Id("b".into())))
        ))
    );
}

#[test]
fn if_with_complex_branches() {
    assert_eq!(
        parse("if x then 1 + 2 else 3 * 4"),
        Expr::parsed(ExprKind::If(
            Box::new(Expr::parsed(ExprKind::Id("x".into()))),
            Box::new(Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Int(1))), Box::new(Expr::parsed(ExprKind::Int(2)))))),
            Box::new(Expr::parsed(ExprKind::Mult(Box::new(Expr::parsed(ExprKind::Int(3))), Box::new(Expr::parsed(ExprKind::Int(4))))))
        ))
    );
}

#[test]
fn nested_if_in_else() {
    assert_eq!(
        parse("if a then 1 else if b then 2 else 3"),
        Expr::parsed(ExprKind::If(
            Box::new(Expr::parsed(ExprKind::Id("a".into()))),
            Box::new(Expr::parsed(ExprKind::Int(1))),
            Box::new(Expr::parsed(ExprKind::If(
                Box::new(Expr::parsed(ExprKind::Id("b".into()))),
                Box::new(Expr::parsed(ExprKind::Int(2))),
                Box::new(Expr::parsed(ExprKind::Int(3)))
            )))
        ))
    );
}

// ─── match ──────────────────────────────────────────────────────────

#[test]
fn match_single_arm() {
    assert_eq!(
        parse("match x { a() -> 1 }"),
        Expr::parsed(ExprKind::Match(
            Box::new(Expr::parsed(ExprKind::Id("x".into()))),
            vec![("a".into(), vec![], Expr::parsed(ExprKind::Int(1)))]
        ))
    );
}

#[test]
fn match_multiple_arms() {
    assert_eq!(
        parse("match x { a() -> 1, b() -> 2 }"),
        Expr::parsed(ExprKind::Match(
            Box::new(Expr::parsed(ExprKind::Id("x".into()))),
            vec![
                ("a".into(), vec![], Expr::parsed(ExprKind::Int(1))),
                ("b".into(), vec![], Expr::parsed(ExprKind::Int(2))),
            ]
        ))
    );
}

#[test]
fn match_with_bindings() {
    assert_eq!(
        parse("match x { some(val: Int) -> val, none() -> 0 }"),
        Expr::parsed(ExprKind::Match(
            Box::new(Expr::parsed(ExprKind::Id("x".into()))),
            vec![
                (
                    "some".into(),
                    vec![Binding::new("val".into(), Type::Int)],
                    Expr::parsed(ExprKind::Id("val".into()))
                ),
                ("none".into(), vec![], Expr::parsed(ExprKind::Int(0))),
            ]
        ))
    );
}

#[test]
fn match_arm_with_expr_body() {
    assert_eq!(
        parse("match x { a(n: Int) -> n + 1 }"),
        Expr::parsed(ExprKind::Match(
            Box::new(Expr::parsed(ExprKind::Id("x".into()))),
            vec![(
                "a".into(),
                vec![Binding::new("n".into(), Type::Int)],
                Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Id("n".into()))), Box::new(Expr::parsed(ExprKind::Int(1)))))
            )]
        ))
    );
}

#[test]
fn match_trailing_comma() {
    // trailing comma should be accepted
    assert_eq!(
        parse("match x { a() -> 1, b() -> 2, }"),
        Expr::parsed(ExprKind::Match(
            Box::new(Expr::parsed(ExprKind::Id("x".into()))),
            vec![
                ("a".into(), vec![], Expr::parsed(ExprKind::Int(1))),
                ("b".into(), vec![], Expr::parsed(ExprKind::Int(2))),
            ]
        ))
    );
}

#[test]
fn match_with_multiple_bindings() {
    assert_eq!(
        parse("match p { point(x: Int, y: Int) -> x + y }"),
        Expr::parsed(ExprKind::Match(
            Box::new(Expr::parsed(ExprKind::Id("p".into()))),
            vec![(
                "point".into(),
                vec![
                    Binding::new("x".into(), Type::Int),
                    Binding::new("y".into(), Type::Int),
                ],
                Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Id("x".into()))), Box::new(Expr::parsed(ExprKind::Id("y".into())))))
            )]
        ))
    );
}

// ─── closures ───────────────────────────────────────────────────────

#[test]
fn simple_closure() {
    assert_eq!(
        parse("fn (x: Int) -> x"),
        Expr::parsed(ExprKind::Fn(Binding::new("x".into(), Type::Int), Box::new(Expr::parsed(ExprKind::Id("x".into())))))
    );
}

#[test]
fn closure_with_body_expr() {
    assert_eq!(
        parse("fn (x: Int) -> x + 1"),
        Expr::parsed(ExprKind::Fn(
            Binding::new("x".into(), Type::Int),
            Box::new(Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Id("x".into()))), Box::new(Expr::parsed(ExprKind::Int(1))))))
        ))
    );
}

#[test]
fn nested_closures() {
    assert_eq!(
        parse("fn (x: Int) -> fn (y: Int) -> x + y"),
        Expr::parsed(ExprKind::Fn(
            Binding::new("x".into(), Type::Int),
            Box::new(Expr::parsed(ExprKind::Fn(
                Binding::new("y".into(), Type::Int),
                Box::new(Expr::parsed(ExprKind::Plus(Box::new(Expr::parsed(ExprKind::Id("x".into()))), Box::new(Expr::parsed(ExprKind::Id("y".into()))))))
            )))
        ))
    );
}

#[test]
fn closure_with_bool_param() {
    assert_eq!(
        parse("fn (b: Bool) -> !b"),
        Expr::parsed(ExprKind::Fn(
            Binding::new("b".into(), Type::Bool),
            Box::new(Expr::parsed(ExprKind::Bang(Box::new(Expr::parsed(ExprKind::Id("b".into()))))))
        ))
    );
}

// ─── complex expressions ────────────────────────────────────────────

#[test]
fn mixed_arithmetic() {
    // 1 + 2 * 3 - 4 = ((1 + (2 * 3)) - 4)
    assert_eq!(
        parse("1 + 2 * 3 - 4"),
        Expr::parsed(ExprKind::Minus(
            Box::new(Expr::parsed(ExprKind::Plus(
                Box::new(Expr::parsed(ExprKind::Int(1))),
                Box::new(Expr::parsed(ExprKind::Mult(Box::new(Expr::parsed(ExprKind::Int(2))), Box::new(Expr::parsed(ExprKind::Int(3))))))
            ))),
            Box::new(Expr::parsed(ExprKind::Int(4)))
        ))
    );
}

#[test]
fn comparison_chain() {
    // a == b != c = (a == b) != c
    assert_eq!(
        parse("a == b != c"),
        Expr::parsed(ExprKind::NotEq(
            Box::new(Expr::parsed(ExprKind::Eq(Box::new(Expr::parsed(ExprKind::Id("a".into()))), Box::new(Expr::parsed(ExprKind::Id("b".into())))))),
            Box::new(Expr::parsed(ExprKind::Id("c".into())))
        ))
    );
}

#[test]
fn full_precedence_chain() {
    // a |> b || c && d == e < f + g * h
    // = a |> (b || (c && (d == (e < (f + (g * h))))))
    assert_eq!(
        parse("a |> b || c && d == e < f + g * h"),
        Expr::parsed(ExprKind::Pipe(
            Box::new(Expr::parsed(ExprKind::Id("a".into()))),
            Box::new(Expr::parsed(ExprKind::Or(
                Box::new(Expr::parsed(ExprKind::Id("b".into()))),
                Box::new(Expr::parsed(ExprKind::And(
                    Box::new(Expr::parsed(ExprKind::Id("c".into()))),
                    Box::new(Expr::parsed(ExprKind::Eq(
                        Box::new(Expr::parsed(ExprKind::Id("d".into()))),
                        Box::new(Expr::parsed(ExprKind::Less(
                            Box::new(Expr::parsed(ExprKind::Id("e".into()))),
                            Box::new(Expr::parsed(ExprKind::Plus(
                                Box::new(Expr::parsed(ExprKind::Id("f".into()))),
                                Box::new(Expr::parsed(ExprKind::Mult(Box::new(Expr::parsed(ExprKind::Id("g".into()))), Box::new(Expr::parsed(ExprKind::Id("h".into()))))))
                            )))
                        )))
                    )))
                )))
            )))
        ))
    );
}

#[test]
fn call_in_pipe() {
    // x |> f(y) — note: pipe binds looser than call, so f(y) is parsed first
    // Actually: the pipe operator gets x and then parses f with pipe precedence.
    // f(y) has higher precedence (Call > Pipe), so it becomes Call(f, [y]).
    // Result: Pipe(x, Call(f, [y]))
    assert_eq!(
        parse("x |> f(y)"),
        Expr::parsed(ExprKind::Pipe(
            Box::new(Expr::parsed(ExprKind::Id("x".into()))),
            Box::new(Expr::parsed(ExprKind::Call(
                Box::new(Expr::parsed(ExprKind::Id("f".into()))),
                vec![Expr::parsed(ExprKind::Id("y".into()))]
            )))
        ))
    );
}

// ─── error cases ────────────────────────────────────────────────────

#[test]
fn error_on_semicolon() {
    let mut p = parser_from(";");
    assert!(super::parse_expr(&mut p).is_err());
}

#[test]
fn error_on_rbrace() {
    let mut p = parser_from("}");
    assert!(super::parse_expr(&mut p).is_err());
}

#[test]
fn error_on_arrow() {
    let mut p = parser_from("->");
    assert!(super::parse_expr(&mut p).is_err());
}

#[test]
fn error_on_eof() {
    let mut p = parser_from("");
    assert!(super::parse_expr(&mut p).is_err());
}
