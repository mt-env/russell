use crate::frontend::lexer::lex;
use crate::frontend::parser::Parser;
use crate::frontend::parser::ast::{ExprKind, ParsedBinding, ParsedExpr, Type};

fn parser_from(input: &str) -> Parser<'_> {
    Parser::new(lex(input))
}

fn parse(input: &str) -> ParsedExpr<'_> {
    let mut p = parser_from(input);
    super::parse_expr(&mut p).unwrap()
}

// ─── atoms ──────────────────────────────────────────────────────────

#[test]
fn int_literal() {
    assert_eq!(parse("42"), ParsedExpr::new(0, ExprKind::Int(42)));
}

#[test]
fn zero() {
    assert_eq!(parse("0"), ParsedExpr::new(0, ExprKind::Int(0)));
}

#[test]
fn float_literal() {
    assert_eq!(parse("3.14"), ParsedExpr::new(0, ExprKind::Float(3.14)));
}

#[test]
fn true_literal() {
    assert_eq!(parse("true"), ParsedExpr::new(0, ExprKind::Bool(true)));
}

#[test]
fn false_literal() {
    assert_eq!(parse("false"), ParsedExpr::new(0, ExprKind::Bool(false)));
}

#[test]
fn identifier() {
    assert_eq!(parse("foo"), ParsedExpr::new(0, ExprKind::Id("foo".into())));
}

// ─── unary operators ────────────────────────────────────────────────

#[test]
fn negate_int() {
    assert_eq!(
        parse("-1"),
        ParsedExpr::new(0, ExprKind::Neg(Box::new(ParsedExpr::new(1, ExprKind::Int(1)))))
    );
}

#[test]
fn negate_identifier() {
    assert_eq!(
        parse("-x"),
        ParsedExpr::new(0, ExprKind::Neg(Box::new(ParsedExpr::new(1, ExprKind::Id("x".into())))))
    );
}

#[test]
fn bang_bool() {
    assert_eq!(
        parse("!true"),
        ParsedExpr::new(0, ExprKind::Bang(Box::new(ParsedExpr::new(1, ExprKind::Bool(true)))))
    );
}

#[test]
fn bang_identifier() {
    assert_eq!(
        parse("!x"),
        ParsedExpr::new(
            0,
            ExprKind::Bang(Box::new(ParsedExpr::new(1, ExprKind::Id("x".into()))))
        )
    );
}

#[test]
fn double_negate() {
    assert_eq!(
        parse("--1"),
        ParsedExpr::new(
            0,
            ExprKind::Neg(Box::new(ParsedExpr::new(
                1,
                ExprKind::Neg(Box::new(ParsedExpr::new(2, ExprKind::Int(1))))
            )))
        )
    );
}

#[test]
fn double_bang() {
    assert_eq!(
        parse("!!true"),
        ParsedExpr::new(
            0,
            ExprKind::Bang(Box::new(ParsedExpr::new(
                1,
                ExprKind::Bang(Box::new(ParsedExpr::new(2, ExprKind::Bool(true))))
            )))
        )
    );
}

// ─── binary operators ───────────────────────────────────────────────

#[test]
fn addition() {
    assert_eq!(
        parse("1 + 2"),
        ParsedExpr::new(
            0,
            ExprKind::Plus(
                Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                Box::new(ParsedExpr::new(4, ExprKind::Int(2)))
            )
        )
    );
}

#[test]
fn subtraction() {
    assert_eq!(
        parse("3 - 1"),
        ParsedExpr::new(
            0,
            ExprKind::Minus(
                Box::new(ParsedExpr::new(0, ExprKind::Int(3))),
                Box::new(ParsedExpr::new(4, ExprKind::Int(1)))
            )
        )
    );
}

#[test]
fn multiplication() {
    assert_eq!(
        parse("2 * 3"),
        ParsedExpr::new(
            0,
            ExprKind::Mult(
                Box::new(ParsedExpr::new(0, ExprKind::Int(2))),
                Box::new(ParsedExpr::new(4, ExprKind::Int(3)))
            )
        )
    );
}

#[test]
fn division() {
    assert_eq!(
        parse("6 / 2"),
        ParsedExpr::new(
            0,
            ExprKind::Div(
                Box::new(ParsedExpr::new(0, ExprKind::Int(6))),
                Box::new(ParsedExpr::new(4, ExprKind::Int(2)))
            )
        )
    );
}

#[test]
fn less_than() {
    assert_eq!(
        parse("a < b"),
        ParsedExpr::new(
            0,
            ExprKind::Less(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(4, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn less_than_or_eq() {
    assert_eq!(
        parse("a <= b"),
        ParsedExpr::new(
            0,
            ExprKind::LessEq(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn greater_than() {
    assert_eq!(
        parse("a > b"),
        ParsedExpr::new(
            0,
            ExprKind::Greater(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(4, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn greater_than_or_eq() {
    assert_eq!(
        parse("a >= b"),
        ParsedExpr::new(
            0,
            ExprKind::GreaterEq(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn equality() {
    assert_eq!(
        parse("a == b"),
        ParsedExpr::new(
            0,
            ExprKind::Eq(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn not_equal() {
    assert_eq!(
        parse("a != b"),
        ParsedExpr::new(
            0,
            ExprKind::NotEq(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn logical_or() {
    assert_eq!(
        parse("a || b"),
        ParsedExpr::new(
            0,
            ExprKind::Or(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn logical_and() {
    assert_eq!(
        parse("a && b"),
        ParsedExpr::new(
            0,
            ExprKind::And(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn pipe() {
    assert_eq!(
        parse("a |> f"),
        ParsedExpr::new(
            0,
            ExprKind::Pipe(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(5, ExprKind::Id("f".into())))
            )
        )
    );
}

// ─── precedence ─────────────────────────────────────────────────────

#[test]
fn mult_before_add() {
    // 1 + 2 * 3 = 1 + (2 * 3)
    assert_eq!(
        parse("1 + 2 * 3"),
        ParsedExpr::new(
            0,
            ExprKind::Plus(
                Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                Box::new(ParsedExpr::new(
                    4,
                    ExprKind::Mult(
                        Box::new(ParsedExpr::new(4, ExprKind::Int(2))),
                        Box::new(ParsedExpr::new(8, ExprKind::Int(3)))
                    )
                ))
            )
        )
    );
}

#[test]
fn mult_before_sub() {
    // 1 - 2 * 3 = 1 - (2 * 3)
    assert_eq!(
        parse("1 - 2 * 3"),
        ParsedExpr::new(
            0,
            ExprKind::Minus(
                Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                Box::new(ParsedExpr::new(
                    4,
                    ExprKind::Mult(
                        Box::new(ParsedExpr::new(4, ExprKind::Int(2))),
                        Box::new(ParsedExpr::new(8, ExprKind::Int(3)))
                    )
                ))
            )
        )
    );
}

#[test]
fn add_before_relational() {
    // a + b < c = (a + b) < c
    assert_eq!(
        parse("a + b < c"),
        ParsedExpr::new(
            0,
            ExprKind::Less(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                        Box::new(ParsedExpr::new(4, ExprKind::Id("b".into())))
                    )
                )),
                Box::new(ParsedExpr::new(8, ExprKind::Id("c".into())))
            )
        )
    );
}

#[test]
fn relational_before_equality() {
    // a < b == c = (a < b) == c
    assert_eq!(
        parse("a < b == c"),
        ParsedExpr::new(
            0,
            ExprKind::Eq(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Less(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                        Box::new(ParsedExpr::new(4, ExprKind::Id("b".into())))
                    )
                )),
                Box::new(ParsedExpr::new(9, ExprKind::Id("c".into())))
            )
        )
    );
}

#[test]
fn equality_before_and() {
    // a == b && c = (a == b) && c
    assert_eq!(
        parse("a == b && c"),
        ParsedExpr::new(
            0,
            ExprKind::And(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Eq(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                        Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
                    )
                )),
                Box::new(ParsedExpr::new(10, ExprKind::Id("c".into())))
            )
        )
    );
}

#[test]
fn and_before_or() {
    // a && b || c = (a && b) || c
    assert_eq!(
        parse("a && b || c"),
        ParsedExpr::new(
            0,
            ExprKind::Or(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::And(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                        Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
                    )
                )),
                Box::new(ParsedExpr::new(10, ExprKind::Id("c".into())))
            )
        )
    );
}

#[test]
fn or_before_pipe() {
    // a || b |> f = (a || b) |> f
    assert_eq!(
        parse("a || b |> f"),
        ParsedExpr::new(
            0,
            ExprKind::Pipe(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Or(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                        Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
                    )
                )),
                Box::new(ParsedExpr::new(10, ExprKind::Id("f".into())))
            )
        )
    );
}

#[test]
fn unary_binds_tighter_than_mult() {
    // -1 * 2 = (-1) * 2
    assert_eq!(
        parse("-1 * 2"),
        ParsedExpr::new(
            0,
            ExprKind::Mult(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Neg(Box::new(ParsedExpr::new(1, ExprKind::Int(1))))
                )),
                Box::new(ParsedExpr::new(5, ExprKind::Int(2)))
            )
        )
    );
}

#[test]
fn unary_binds_tighter_than_add() {
    // -1 + 2 = (-1) + 2
    assert_eq!(
        parse("-1 + 2"),
        ParsedExpr::new(
            0,
            ExprKind::Plus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Neg(Box::new(ParsedExpr::new(1, ExprKind::Int(1))))
                )),
                Box::new(ParsedExpr::new(5, ExprKind::Int(2)))
            )
        )
    );
}

#[test]
fn bang_binds_tighter_than_and() {
    // !a && b = (!a) && b
    assert_eq!(
        parse("!a && b"),
        ParsedExpr::new(
            0,
            ExprKind::And(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Bang(Box::new(ParsedExpr::new(1, ExprKind::Id("a".into()))))
                )),
                Box::new(ParsedExpr::new(6, ExprKind::Id("b".into())))
            )
        )
    );
}

// ─── associativity (left-to-right) ──────────────────────────────────

#[test]
fn addition_left_assoc() {
    // 1 + 2 + 3 = (1 + 2) + 3
    assert_eq!(
        parse("1 + 2 + 3"),
        ParsedExpr::new(
            0,
            ExprKind::Plus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                        Box::new(ParsedExpr::new(4, ExprKind::Int(2)))
                    )
                )),
                Box::new(ParsedExpr::new(8, ExprKind::Int(3)))
            )
        )
    );
}

#[test]
fn subtraction_left_assoc() {
    // 5 - 3 - 1 = (5 - 3) - 1
    assert_eq!(
        parse("5 - 3 - 1"),
        ParsedExpr::new(
            0,
            ExprKind::Minus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Minus(
                        Box::new(ParsedExpr::new(0, ExprKind::Int(5))),
                        Box::new(ParsedExpr::new(4, ExprKind::Int(3)))
                    )
                )),
                Box::new(ParsedExpr::new(8, ExprKind::Int(1)))
            )
        )
    );
}

#[test]
fn multiplication_left_assoc() {
    // 2 * 3 * 4 = (2 * 3) * 4
    assert_eq!(
        parse("2 * 3 * 4"),
        ParsedExpr::new(
            0,
            ExprKind::Mult(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Mult(
                        Box::new(ParsedExpr::new(0, ExprKind::Int(2))),
                        Box::new(ParsedExpr::new(4, ExprKind::Int(3)))
                    )
                )),
                Box::new(ParsedExpr::new(8, ExprKind::Int(4)))
            )
        )
    );
}

#[test]
fn pipe_left_assoc() {
    // x |> f |> g = (x |> f) |> g
    assert_eq!(
        parse("x |> f |> g"),
        ParsedExpr::new(
            0,
            ExprKind::Pipe(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Pipe(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("x".into()))),
                        Box::new(ParsedExpr::new(5, ExprKind::Id("f".into())))
                    )
                )),
                Box::new(ParsedExpr::new(10, ExprKind::Id("g".into())))
            )
        )
    );
}

#[test]
fn mixed_add_sub_left_assoc() {
    // 1 + 2 - 3 = (1 + 2) - 3
    assert_eq!(
        parse("1 + 2 - 3"),
        ParsedExpr::new(
            0,
            ExprKind::Minus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                        Box::new(ParsedExpr::new(4, ExprKind::Int(2)))
                    )
                )),
                Box::new(ParsedExpr::new(8, ExprKind::Int(3)))
            )
        )
    );
}

// ─── parenthesized expressions ──────────────────────────────────────

#[test]
fn parens_identity() {
    assert_eq!(parse("(42)"), ParsedExpr::new(0, ExprKind::Int(42)));
}

#[test]
fn parens_override_precedence() {
    // (1 + 2) * 3
    assert_eq!(
        parse("(1 + 2) * 3"),
        ParsedExpr::new(
            0,
            ExprKind::Mult(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(1, ExprKind::Int(1))),
                        Box::new(ParsedExpr::new(5, ExprKind::Int(2)))
                    )
                )),
                Box::new(ParsedExpr::new(10, ExprKind::Int(3)))
            )
        )
    );
}

#[test]
fn nested_parens() {
    assert_eq!(parse("((1))"), ParsedExpr::new(0, ExprKind::Int(1)));
}

#[test]
fn parens_in_right_operand() {
    // 2 * (3 + 4)
    assert_eq!(
        parse("2 * (3 + 4)"),
        ParsedExpr::new(
            0,
            ExprKind::Mult(
                Box::new(ParsedExpr::new(0, ExprKind::Int(2))),
                Box::new(ParsedExpr::new(
                    4,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(5, ExprKind::Int(3))),
                        Box::new(ParsedExpr::new(9, ExprKind::Int(4)))
                    )
                ))
            )
        )
    );
}

// ─── function calls ─────────────────────────────────────────────────

#[test]
fn call_no_args() {
    assert_eq!(
        parse("f()"),
        ParsedExpr::new(
            0,
            ExprKind::Call(Box::new(ParsedExpr::new(0, ExprKind::Id("f".into()))), vec![])
        )
    );
}

#[test]
fn call_one_arg() {
    assert_eq!(
        parse("f(1)"),
        ParsedExpr::new(
            0,
            ExprKind::Call(
                Box::new(ParsedExpr::new(0, ExprKind::Id("f".into()))),
                vec![ParsedExpr::new(2, ExprKind::Int(1))]
            )
        )
    );
}

#[test]
fn call_multiple_args() {
    assert_eq!(
        parse("f(1, 2, 3)"),
        ParsedExpr::new(
            0,
            ExprKind::Call(
                Box::new(ParsedExpr::new(0, ExprKind::Id("f".into()))),
                vec![
                    ParsedExpr::new(2, ExprKind::Int(1)),
                    ParsedExpr::new(5, ExprKind::Int(2)),
                    ParsedExpr::new(8, ExprKind::Int(3))
                ]
            )
        )
    );
}

#[test]
fn call_with_expr_arg() {
    assert_eq!(
        parse("f(1 + 2)"),
        ParsedExpr::new(
            0,
            ExprKind::Call(
                Box::new(ParsedExpr::new(0, ExprKind::Id("f".into()))),
                vec![ParsedExpr::new(
                    2,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(2, ExprKind::Int(1))),
                        Box::new(ParsedExpr::new(6, ExprKind::Int(2)))
                    )
                )]
            )
        )
    );
}

#[test]
fn chained_calls() {
    // f(x)(y) = Call(Call(f, [x]), [y])
    assert_eq!(
        parse("f(x)(y)"),
        ParsedExpr::new(
            0,
            ExprKind::Call(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Call(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("f".into()))),
                        vec![ParsedExpr::new(2, ExprKind::Id("x".into()))]
                    )
                )),
                vec![ParsedExpr::new(5, ExprKind::Id("y".into()))]
            )
        )
    );
}

#[test]
fn call_in_binary_expr() {
    // f(x) + 1
    assert_eq!(
        parse("f(x) + 1"),
        ParsedExpr::new(
            0,
            ExprKind::Plus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Call(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("f".into()))),
                        vec![ParsedExpr::new(2, ExprKind::Id("x".into()))]
                    )
                )),
                Box::new(ParsedExpr::new(7, ExprKind::Int(1)))
            )
        )
    );
}

#[test]
fn binary_expr_then_call() {
    // 1 + f(x)
    assert_eq!(
        parse("1 + f(x)"),
        ParsedExpr::new(
            0,
            ExprKind::Plus(
                Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                Box::new(ParsedExpr::new(
                    4,
                    ExprKind::Call(
                        Box::new(ParsedExpr::new(4, ExprKind::Id("f".into()))),
                        vec![ParsedExpr::new(6, ExprKind::Id("x".into()))]
                    )
                ))
            )
        )
    );
}

#[test]
fn negate_call() {
    // -f(x) = Neg(Call(f, [x]))
    assert_eq!(
        parse("-f(x)"),
        ParsedExpr::new(
            0,
            ExprKind::Neg(Box::new(ParsedExpr::new(
                1,
                ExprKind::Call(
                    Box::new(ParsedExpr::new(1, ExprKind::Id("f".into()))),
                    vec![ParsedExpr::new(3, ExprKind::Id("x".into()))]
                )
            )))
        )
    );
}

// ─── if-then-else ───────────────────────────────────────────────────

#[test]
fn simple_if() {
    assert_eq!(
        parse("if true then 1 else 2"),
        ParsedExpr::new(
            0,
            ExprKind::If(
                Box::new(ParsedExpr::new(3, ExprKind::Bool(true))),
                Box::new(ParsedExpr::new(13, ExprKind::Int(1))),
                Box::new(ParsedExpr::new(20, ExprKind::Int(2)))
            )
        )
    );
}

#[test]
fn if_with_condition_expr() {
    assert_eq!(
        parse("if a == b then a else b"),
        ParsedExpr::new(
            0,
            ExprKind::If(
                Box::new(ParsedExpr::new(
                    3,
                    ExprKind::Eq(
                        Box::new(ParsedExpr::new(3, ExprKind::Id("a".into()))),
                        Box::new(ParsedExpr::new(8, ExprKind::Id("b".into())))
                    )
                )),
                Box::new(ParsedExpr::new(15, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(22, ExprKind::Id("b".into())))
            )
        )
    );
}

#[test]
fn if_with_complex_branches() {
    assert_eq!(
        parse("if x then 1 + 2 else 3 * 4"),
        ParsedExpr::new(
            0,
            ExprKind::If(
                Box::new(ParsedExpr::new(3, ExprKind::Id("x".into()))),
                Box::new(ParsedExpr::new(
                    10,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(10, ExprKind::Int(1))),
                        Box::new(ParsedExpr::new(14, ExprKind::Int(2)))
                    )
                )),
                Box::new(ParsedExpr::new(
                    21,
                    ExprKind::Mult(
                        Box::new(ParsedExpr::new(21, ExprKind::Int(3))),
                        Box::new(ParsedExpr::new(25, ExprKind::Int(4)))
                    )
                ))
            )
        )
    );
}

#[test]
fn nested_if_in_else() {
    assert_eq!(
        parse("if a then 1 else if b then 2 else 3"),
        ParsedExpr::new(
            0,
            ExprKind::If(
                Box::new(ParsedExpr::new(3, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(10, ExprKind::Int(1))),
                Box::new(ParsedExpr::new(
                    17,
                    ExprKind::If(
                        Box::new(ParsedExpr::new(20, ExprKind::Id("b".into()))),
                        Box::new(ParsedExpr::new(27, ExprKind::Int(2))),
                        Box::new(ParsedExpr::new(34, ExprKind::Int(3)))
                    )
                ))
            )
        )
    );
}

// ─── match ──────────────────────────────────────────────────────────

#[test]
fn match_single_arm() {
    assert_eq!(
        parse("match x { a() -> 1 }"),
        ParsedExpr::new(
            0,
            ExprKind::Match(
                Box::new(ParsedExpr::new(6, ExprKind::Id("x".into()))),
                vec![("a".into(), vec![], ParsedExpr::new(17, ExprKind::Int(1)))]
            )
        )
    );
}

#[test]
fn match_multiple_arms() {
    assert_eq!(
        parse("match x { a() -> 1, b() -> 2 }"),
        ParsedExpr::new(
            0,
            ExprKind::Match(
                Box::new(ParsedExpr::new(6, ExprKind::Id("x".into()))),
                vec![
                    ("a".into(), vec![], ParsedExpr::new(17, ExprKind::Int(1))),
                    ("b".into(), vec![], ParsedExpr::new(27, ExprKind::Int(2))),
                ]
            )
        )
    );
}

#[test]
fn match_with_bindings() {
    assert_eq!(
        parse("match x { some(val: Int) -> val, none() -> 0 }"),
        ParsedExpr::new(
            0,
            ExprKind::Match(
                Box::new(ParsedExpr::new(6, ExprKind::Id("x".into()))),
                vec![
                    (
                        "some".into(),
                        vec![ParsedBinding::new(15, "val".into(), Type::Int)],
                        ParsedExpr::new(28, ExprKind::Id("val".into()))
                    ),
                    ("none".into(), vec![], ParsedExpr::new(43, ExprKind::Int(0))),
                ]
            )
        )
    );
}

#[test]
fn match_arm_with_expr_body() {
    assert_eq!(
        parse("match x { a(n: Int) -> n + 1 }"),
        ParsedExpr::new(
            0,
            ExprKind::Match(
                Box::new(ParsedExpr::new(6, ExprKind::Id("x".into()))),
                vec![(
                    "a".into(),
                    vec![ParsedBinding::new(12, "n".into(), Type::Int)],
                    ParsedExpr::new(
                        23,
                        ExprKind::Plus(
                            Box::new(ParsedExpr::new(23, ExprKind::Id("n".into()))),
                            Box::new(ParsedExpr::new(27, ExprKind::Int(1)))
                        )
                    )
                )]
            )
        )
    );
}

#[test]
fn match_trailing_comma() {
    // trailing comma should be accepted
    assert_eq!(
        parse("match x { a() -> 1, b() -> 2, }"),
        ParsedExpr::new(
            0,
            ExprKind::Match(
                Box::new(ParsedExpr::new(6, ExprKind::Id("x".into()))),
                vec![
                    ("a".into(), vec![], ParsedExpr::new(17, ExprKind::Int(1))),
                    ("b".into(), vec![], ParsedExpr::new(27, ExprKind::Int(2))),
                ]
            )
        )
    );
}

#[test]
fn match_with_multiple_bindings() {
    assert_eq!(
        parse("match p { point(x: Int, y: Int) -> x + y }"),
        ParsedExpr::new(
            0,
            ExprKind::Match(
                Box::new(ParsedExpr::new(6, ExprKind::Id("p".into()))),
                vec![(
                    "point".into(),
                    vec![
                        ParsedBinding::new(16, "x".into(), Type::Int),
                        ParsedBinding::new(24, "y".into(), Type::Int),
                    ],
                    ParsedExpr::new(
                        35,
                        ExprKind::Plus(
                            Box::new(ParsedExpr::new(35, ExprKind::Id("x".into()))),
                            Box::new(ParsedExpr::new(39, ExprKind::Id("y".into())))
                        )
                    )
                )]
            )
        )
    );
}

// ─── closures ───────────────────────────────────────────────────────

#[test]
fn simple_closure() {
    assert_eq!(
        parse("fn (x: Int) -> x"),
        ParsedExpr::new(
            0,
            ExprKind::Fn(
                ParsedBinding::new(4, "x".into(), Type::Int),
                Box::new(ParsedExpr::new(15, ExprKind::Id("x".into())))
            )
        )
    );
}

#[test]
fn closure_with_body_expr() {
    assert_eq!(
        parse("fn (x: Int) -> x + 1"),
        ParsedExpr::new(
            0,
            ExprKind::Fn(
                ParsedBinding::new(4, "x".into(), Type::Int),
                Box::new(ParsedExpr::new(
                    15,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(15, ExprKind::Id("x".into()))),
                        Box::new(ParsedExpr::new(19, ExprKind::Int(1)))
                    )
                ))
            )
        )
    );
}

#[test]
fn nested_closures() {
    assert_eq!(
        parse("fn (x: Int) -> fn (y: Int) -> x + y"),
        ParsedExpr::new(
            0,
            ExprKind::Fn(
                ParsedBinding::new(4, "x".into(), Type::Int),
                Box::new(ParsedExpr::new(
                    15,
                    ExprKind::Fn(
                        ParsedBinding::new(19, "y".into(), Type::Int),
                        Box::new(ParsedExpr::new(
                            30,
                            ExprKind::Plus(
                                Box::new(ParsedExpr::new(30, ExprKind::Id("x".into()))),
                                Box::new(ParsedExpr::new(34, ExprKind::Id("y".into())))
                            )
                        ))
                    )
                ))
            )
        )
    );
}

#[test]
fn closure_with_bool_param() {
    assert_eq!(
        parse("fn (b: Bool) -> !b"),
        ParsedExpr::new(
            0,
            ExprKind::Fn(
                ParsedBinding::new(4, "b".into(), Type::Bool),
                Box::new(ParsedExpr::new(
                    16,
                    ExprKind::Bang(Box::new(ParsedExpr::new(17, ExprKind::Id("b".into()))))
                ))
            )
        )
    );
}

// ─── complex expressions ────────────────────────────────────────────

#[test]
fn mixed_arithmetic() {
    // 1 + 2 * 3 - 4 = ((1 + (2 * 3)) - 4)
    assert_eq!(
        parse("1 + 2 * 3 - 4"),
        ParsedExpr::new(
            0,
            ExprKind::Minus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                        Box::new(ParsedExpr::new(
                            4,
                            ExprKind::Mult(
                                Box::new(ParsedExpr::new(4, ExprKind::Int(2))),
                                Box::new(ParsedExpr::new(8, ExprKind::Int(3)))
                            )
                        ))
                    )
                )),
                Box::new(ParsedExpr::new(12, ExprKind::Int(4)))
            )
        )
    );
}

#[test]
fn comparison_chain() {
    // a == b != c = (a == b) != c
    assert_eq!(
        parse("a == b != c"),
        ParsedExpr::new(
            0,
            ExprKind::NotEq(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Eq(
                        Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                        Box::new(ParsedExpr::new(5, ExprKind::Id("b".into())))
                    )
                )),
                Box::new(ParsedExpr::new(10, ExprKind::Id("c".into())))
            )
        )
    );
}

#[test]
fn full_precedence_chain() {
    // a |> b || c && d == e < f + g * h
    // = a |> (b || (c && (d == (e < (f + (g * h))))))
    assert_eq!(
        parse("a |> b || c && d == e < f + g * h"),
        ParsedExpr::new(
            0,
            ExprKind::Pipe(
                Box::new(ParsedExpr::new(0, ExprKind::Id("a".into()))),
                Box::new(ParsedExpr::new(
                    5,
                    ExprKind::Or(
                        Box::new(ParsedExpr::new(5, ExprKind::Id("b".into()))),
                        Box::new(ParsedExpr::new(
                            10,
                            ExprKind::And(
                                Box::new(ParsedExpr::new(10, ExprKind::Id("c".into()))),
                                Box::new(ParsedExpr::new(
                                    15,
                                    ExprKind::Eq(
                                        Box::new(ParsedExpr::new(15, ExprKind::Id("d".into()))),
                                        Box::new(ParsedExpr::new(
                                            20,
                                            ExprKind::Less(
                                                Box::new(ParsedExpr::new(20, ExprKind::Id("e".into()))),
                                                Box::new(ParsedExpr::new(
                                                    24,
                                                    ExprKind::Plus(
                                                        Box::new(ParsedExpr::new(24, ExprKind::Id("f".into()))),
                                                        Box::new(ParsedExpr::new(
                                                            28,
                                                            ExprKind::Mult(
                                                                Box::new(ParsedExpr::new(28, ExprKind::Id("g".into()))),
                                                                Box::new(ParsedExpr::new(32, ExprKind::Id("h".into())))
                                                            )
                                                        ))
                                                    )
                                                ))
                                            )
                                        ))
                                    )
                                ))
                            )
                        ))
                    )
                ))
            )
        )
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
        ParsedExpr::new(
            0,
            ExprKind::Pipe(
                Box::new(ParsedExpr::new(0, ExprKind::Id("x".into()))),
                Box::new(ParsedExpr::new(
                    5,
                    ExprKind::Call(
                        Box::new(ParsedExpr::new(5, ExprKind::Id("f".into()))),
                        vec![ParsedExpr::new(7, ExprKind::Id("y".into()))]
                    )
                ))
            )
        )
    );
}

// ─── floating-point operators ────────────────────────────────────

#[test]
fn float_addition() {
    assert_eq!(
        parse("1.0 +. 2.5"),
        ParsedExpr::new(
            0,
            ExprKind::FPlus(
                Box::new(ParsedExpr::new(0, ExprKind::Float(1.0))),
                Box::new(ParsedExpr::new(7, ExprKind::Float(2.5)))
            )
        )
    );
}

#[test]
fn float_subtraction() {
    assert_eq!(
        parse("3.14 -. 1.0"),
        ParsedExpr::new(
            0,
            ExprKind::FMinus(
                Box::new(ParsedExpr::new(0, ExprKind::Float(3.14))),
                Box::new(ParsedExpr::new(8, ExprKind::Float(1.0)))
            )
        )
    );
}

#[test]
fn float_multiplication() {
    assert_eq!(
        parse("2.0 *. 3.5"),
        ParsedExpr::new(
            0,
            ExprKind::FMult(
                Box::new(ParsedExpr::new(0, ExprKind::Float(2.0))),
                Box::new(ParsedExpr::new(7, ExprKind::Float(3.5)))
            )
        )
    );
}

#[test]
fn float_division() {
    assert_eq!(
        parse("10.0 /. 2.5"),
        ParsedExpr::new(
            0,
            ExprKind::FDiv(
                Box::new(ParsedExpr::new(0, ExprKind::Float(10.0))),
                Box::new(ParsedExpr::new(8, ExprKind::Float(2.5)))
            )
        )
    );
}

#[test]
fn mixed_float_ops() {
    // 1.0 +. 2.0 *. 3.0 = 1.0 +. (2.0 *. 3.0)
    assert_eq!(
        parse("1.0 +. 2.0 *. 3.0"),
        ParsedExpr::new(
            0,
            ExprKind::FPlus(
                Box::new(ParsedExpr::new(0, ExprKind::Float(1.0))),
                Box::new(ParsedExpr::new(
                    7,
                    ExprKind::FMult(
                        Box::new(ParsedExpr::new(7, ExprKind::Float(2.0))),
                        Box::new(ParsedExpr::new(14, ExprKind::Float(3.0)))
                    )
                ))
            )
        )
    );
}

#[test]
fn float_ops_left_assoc() {
    // 1.0 +. 2.0 +. 3.0 = (1.0 +. 2.0) +. 3.0
    assert_eq!(
        parse("1.0 +. 2.0 +. 3.0"),
        ParsedExpr::new(
            0,
            ExprKind::FPlus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::FPlus(
                        Box::new(ParsedExpr::new(0, ExprKind::Float(1.0))),
                        Box::new(ParsedExpr::new(7, ExprKind::Float(2.0)))
                    )
                )),
                Box::new(ParsedExpr::new(14, ExprKind::Float(3.0)))
            )
        )
    );
}

#[test]
fn float_and_int_ops_mixed() {
    // x + 1 +. y tests that both regular and float ops can be parsed
    assert_eq!(
        parse("1 + 2 +. 3.0"),
        ParsedExpr::new(
            0,
            ExprKind::FPlus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                        Box::new(ParsedExpr::new(4, ExprKind::Int(2)))
                    )
                )),
                Box::new(ParsedExpr::new(9, ExprKind::Float(3.0)))
            )
        )
    );
}

#[test]
fn int_operands_with_float_op() {
    // 1 +. 2 should parse (type error only at interpreter)
    assert_eq!(
        parse("1 +. 2"),
        ParsedExpr::new(
            0,
            ExprKind::FPlus(
                Box::new(ParsedExpr::new(0, ExprKind::Int(1))),
                Box::new(ParsedExpr::new(5, ExprKind::Int(2)))
            )
        )
    );
}

#[test]
fn float_operands_with_int_op() {
    // 1.0 + 2.0 should parse
    assert_eq!(
        parse("1.0 + 2.0"),
        ParsedExpr::new(
            0,
            ExprKind::Plus(
                Box::new(ParsedExpr::new(0, ExprKind::Float(1.0))),
                Box::new(ParsedExpr::new(6, ExprKind::Float(2.0)))
            )
        )
    );
}

#[test]
fn mixed_float_int_all_ops() {
    // 1.0 + 2 -. 3.0 tests that all combinations parse regardless of operand types
    assert_eq!(
        parse("1.0 + 2 -. 3.0"),
        ParsedExpr::new(
            0,
            ExprKind::FMinus(
                Box::new(ParsedExpr::new(
                    0,
                    ExprKind::Plus(
                        Box::new(ParsedExpr::new(0, ExprKind::Float(1.0))),
                        Box::new(ParsedExpr::new(6, ExprKind::Int(2)))
                    )
                )),
                Box::new(ParsedExpr::new(11, ExprKind::Float(3.0)))
            )
        )
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
