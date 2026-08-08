use super::lex;
use super::token::{LexError, SpannedLexError, SpannedToken, Token};

// empty/EoF

#[test]
fn empty_input() {
    assert_eq!(lex(""), Ok(vec![SpannedToken::new(Token::EoF, 0)]));
}

#[test]
fn whitespace_only() {
    assert_eq!(lex("   \t\n  "), Ok(vec![SpannedToken::new(Token::EoF, 7)]));
}

// integer literals

#[test]
fn single_digit_int() {
    assert_eq!(
        lex("5"),
        Ok(vec![
            SpannedToken::new(Token::Int(5), 0),
            SpannedToken::new(Token::EoF, "5".len()),
        ]),
    );
}

#[test]
fn multi_digit_int() {
    assert_eq!(
        lex("12345"),
        Ok(vec![
            SpannedToken::new(Token::Int(12345), 0),
            SpannedToken::new(Token::EoF, "12345".len()),
        ]),
    );
}

#[test]
fn zero() {
    assert_eq!(
        lex("0"),
        Ok(vec![
            SpannedToken::new(Token::Int(0), 0),
            SpannedToken::new(Token::EoF, "0".len()),
        ]),
    );
}

// float literals

#[test]
fn simple_float() {
    assert_eq!(
        lex("3.14"),
        Ok(vec![
            SpannedToken::new(Token::Float(3.14), 0),
            SpannedToken::new(Token::EoF, "3.14".len()),
        ]),
    );
}

#[test]
fn float_leading_zero() {
    assert_eq!(
        lex("0.5"),
        Ok(vec![
            SpannedToken::new(Token::Float(0.5), 0),
            SpannedToken::new(Token::EoF, "0.5".len()),
        ]),
    );
}

#[test]
fn float_trailing_dot() {
    assert_eq!(
        lex("1."),
        Ok(vec![
            SpannedToken::new(Token::Float(1.0), 0),
            SpannedToken::new(Token::EoF, "1.".len()),
        ]),
    );
}

#[test]
fn float_trailing_dot_before_identifier() {
    assert_eq!(
        lex("1.foo"),
        Ok(vec![
            SpannedToken::new(Token::Float(1.0), 0),
            SpannedToken::new(Token::Id("foo"), 2),
            SpannedToken::new(Token::EoF, 5),
        ]),
    );
}

#[test]
fn number_with_two_dots() {
    assert_eq!(
        lex("1.2.3"),
        Err(vec![SpannedLexError::new(
            LexError::InvalidFloat("1.2.3"),
            0
        )])
    );
}

// boolean literals

#[test]
fn true_literal() {
    assert_eq!(
        lex("true"),
        Ok(vec![
            SpannedToken::new(Token::Bool(true), 0),
            SpannedToken::new(Token::EoF, "true".len()),
        ]),
    );
}

#[test]
fn false_literal() {
    assert_eq!(
        lex("false"),
        Ok(vec![
            SpannedToken::new(Token::Bool(false), 0),
            SpannedToken::new(Token::EoF, "false".len()),
        ]),
    );
}

// keywords

#[test]
fn all_keywords() {
    let cases = [
        ("echo", Token::Echo),
        ("else", Token::Else),
        ("fn", Token::Fn),
        ("if", Token::If),
        ("let", Token::Let),
        ("match", Token::Match),
        ("read", Token::Read),
        ("return", Token::Return),
        ("then", Token::Then),
        ("typedef", Token::Typedef),
    ];
    for (input, expected) in cases {
        assert_eq!(
            lex(input),
            Ok(vec![
                SpannedToken::new(expected, 0),
                SpannedToken::new(Token::EoF, input.len()),
            ]),
        );
    }
}

#[test]
fn keyword_prefix_is_identifier() {
    // "letters" starts with "let" but should not be a keyword
    assert_eq!(
        lex("letters"),
        Ok(vec![
            SpannedToken::new(Token::Id("letters"), 0),
            SpannedToken::new(Token::EoF, "letters".len()),
        ]),
    );
}

#[test]
fn keyword_with_suffix_is_identifier() {
    assert_eq!(
        lex("iffoo"),
        Ok(vec![
            SpannedToken::new(Token::Id("iffoo"), 0),
            SpannedToken::new(Token::EoF, "iffoo".len()),
        ]),
    );
    assert_eq!(
        lex("return_value"),
        Ok(vec![
            SpannedToken::new(Token::Id("return_value"), 0),
            SpannedToken::new(Token::EoF, "return_value".len()),
        ]),
    );
}

// identifiers

#[test]
fn simple_identifier() {
    assert_eq!(
        lex("foo"),
        Ok(vec![
            SpannedToken::new(Token::Id("foo"), 0),
            SpannedToken::new(Token::EoF, "foo".len()),
        ]),
    );
}

#[test]
fn identifier_with_underscores() {
    assert_eq!(
        lex("my_var_name"),
        Ok(vec![
            SpannedToken::new(Token::Id("my_var_name"), 0),
            SpannedToken::new(Token::EoF, "my_var_name".len()),
        ]),
    );
}

#[test]
fn identifier_with_digits() {
    assert_eq!(
        lex("x2"),
        Ok(vec![
            SpannedToken::new(Token::Id("x2"), 0),
            SpannedToken::new(Token::EoF, "x2".len()),
        ]),
    );
}

// type identifiers

#[test]
fn builtin_type_keywords() {
    assert_eq!(
        lex("Int"),
        Ok(vec![
            SpannedToken::new(Token::IntType, 0),
            SpannedToken::new(Token::EoF, "Int".len()),
        ]),
    );
    assert_eq!(
        lex("Float"),
        Ok(vec![
            SpannedToken::new(Token::FloatType, 0),
            SpannedToken::new(Token::EoF, "Float".len()),
        ]),
    );
    assert_eq!(
        lex("Bool"),
        Ok(vec![
            SpannedToken::new(Token::BoolType, 0),
            SpannedToken::new(Token::EoF, "Bool".len()),
        ]),
    );
}

#[test]
fn custom_type_identifier() {
    assert_eq!(
        lex("MyType"),
        Ok(vec![
            SpannedToken::new(Token::TypeId("MyType"), 0),
            SpannedToken::new(Token::EoF, "MyType".len()),
        ]),
    );
}

#[test]
fn type_id_allows_digits_and_underscores() {
    assert_eq!(
        lex("Vec2_Type"),
        Ok(vec![
            SpannedToken::new(Token::TypeId("Vec2_Type"), 0),
            SpannedToken::new(Token::EoF, "Vec2_Type".len()),
        ]),
    );
}

#[test]
fn type_param_single_letter() {
    assert_eq!(
        lex("'a"),
        Ok(vec![
            SpannedToken::new(Token::TypeParam("'a"), 0),
            SpannedToken::new(Token::EoF, "'a".len()),
        ]),
    );
}

#[test]
fn type_param_stops_at_non_alpha() {
    assert_eq!(
        lex("'abc1"),
        Ok(vec![
            SpannedToken::new(Token::TypeParam("'abc"), 0),
            SpannedToken::new(Token::Int(1), 4),
            SpannedToken::new(Token::EoF, 5),
        ]),
    );
}

#[test]
fn invalid_type_param_without_name() {
    assert_eq!(
        lex("'"),
        Err(vec![SpannedLexError::new(LexError::InvalidChar('\''), 0)])
    );
}

#[test]
fn invalid_type_param_with_uppercase() {
    assert_eq!(
        lex("'Abc"),
        Err(vec![SpannedLexError::new(LexError::InvalidChar('\''), 0)])
    );
}

#[test]
fn invalid_type_param_without_lowercase() {
    assert_eq!(
        lex("'_x"),
        Err(vec![
            SpannedLexError::new(LexError::InvalidChar('\''), 0),
            SpannedLexError::new(LexError::InvalidChar('_'), 1),
        ])
    );
}

// ─── One-character operators / punctuation ──────────────────────────────────

#[test]
fn single_char_operators() {
    let cases = [
        ("=", Token::Assign),
        ("!", Token::Not),
        ("(", Token::LParen),
        (")", Token::RParen),
        ("*", Token::Times),
        ("+", Token::Plus),
        (",", Token::Comma),
        ("-", Token::Minus),
        ("/", Token::Divide),
        (":", Token::Colon),
        (";", Token::Semicolon),
        ("<", Token::LessThan),
        (">", Token::GreaterThan),
        ("{", Token::LBrace),
        ("}", Token::RBrace),
    ];
    for (input, expected) in cases {
        assert_eq!(
            lex(input),
            Ok(vec![
                SpannedToken::new(expected, 0),
                SpannedToken::new(Token::EoF, input.len()),
            ]),
        );
    }
}

// two-character operators

#[test]
fn two_char_operators() {
    let cases = [
        ("!=", Token::NotEq),
        ("&&", Token::And),
        ("->", Token::Arrow),
        ("<=", Token::LessThanOrEq),
        ("==", Token::Eq),
        (">=", Token::GreaterThanOrEq),
        ("|>", Token::Pipe),
        ("||", Token::Or),
        ("+.", Token::FPlus),
        ("-.", Token::FMinus),
        ("*.", Token::FTimes),
        ("/.", Token::FDivide),
        ("<.", Token::FLessThan),
        (">.", Token::FGreaterThan),
        ("<=.", Token::FLessThanOrEq),
        (">=.", Token::FGreaterThanOrEq),
    ];
    for (input, expected) in cases {
        assert_eq!(
            lex(input),
            Ok(vec![
                SpannedToken::new(expected, 0),
                SpannedToken::new(Token::EoF, input.len()),
            ]),
        );
    }
}

#[test]
fn two_char_op_preferred_over_one_char() {
    // "!=" should be NotEq, not Not followed by Assign
    assert_eq!(
        lex("!="),
        Ok(vec![
            SpannedToken::new(Token::NotEq, 0),
            SpannedToken::new(Token::EoF, "!=".len()),
        ]),
    );

    // "<=" should be LessThanOrEq, not LessThan followed by Assign
    assert_eq!(
        lex("<="),
        Ok(vec![
            SpannedToken::new(Token::LessThanOrEq, 0),
            SpannedToken::new(Token::EoF, "<=".len()),
        ]),
    );
}

#[test]
fn three_char_op_preferred_over_shorter_prefixes() {
    assert_eq!(
        lex("<=."),
        Ok(vec![
            SpannedToken::new(Token::FLessThanOrEq, 0),
            SpannedToken::new(Token::EoF, "<=.".len()),
        ]),
    );
    assert_eq!(
        lex(">=."),
        Ok(vec![
            SpannedToken::new(Token::FGreaterThanOrEq, 0),
            SpannedToken::new(Token::EoF, ">=.".len()),
        ]),
    );
}

// whitespace handling

#[test]
fn spaces_between_tokens() {
    assert_eq!(
        lex("1 + 2"),
        Ok(vec![
            SpannedToken::new(Token::Int(1), 0),
            SpannedToken::new(Token::Plus, 2),
            SpannedToken::new(Token::Int(2), 4),
            SpannedToken::new(Token::EoF, 5),
        ]),
    );
}

#[test]
fn tabs_and_newlines() {
    assert_eq!(
        lex("let\n\tx\t=\n5"),
        Ok(vec![
            SpannedToken::new(Token::Let, 0),
            SpannedToken::new(Token::Id("x"), 5),
            SpannedToken::new(Token::Assign, 7),
            SpannedToken::new(Token::Int(5), 9),
            SpannedToken::new(Token::EoF, 10),
        ]),
    );
}

#[test]
fn no_whitespace_between_tokens() {
    assert_eq!(
        lex("1+2"),
        Ok(vec![
            SpannedToken::new(Token::Int(1), 0),
            SpannedToken::new(Token::Plus, 1),
            SpannedToken::new(Token::Int(2), 2),
            SpannedToken::new(Token::EoF, 3),
        ]),
    );
}

// comments

#[test]
fn line_comment_skipped() {
    assert_eq!(
        lex("// this is a comment\n42"),
        Ok(vec![
            SpannedToken::new(Token::Int(42), 21),
            SpannedToken::new(Token::EoF, 23),
        ]),
    );
}

#[test]
fn comment_at_end_of_line() {
    assert_eq!(
        lex("42 // trailing comment"),
        Ok(vec![
            SpannedToken::new(Token::Int(42), 0),
            SpannedToken::new(Token::EoF, 22),
        ]),
    );
}

#[test]
fn multiple_comment_lines() {
    assert_eq!(
        lex("// first\n// second\n// third\n7"),
        Ok(vec![
            SpannedToken::new(Token::Int(7), 28),
            SpannedToken::new(Token::EoF, 29),
        ]),
    );
}

#[test]
fn comment_only_no_newline() {
    assert_eq!(
        lex("// just a comment"),
        Ok(vec![SpannedToken::new(Token::EoF, 17)])
    );
}

#[test]
fn comment_between_tokens() {
    assert_eq!(
        lex("1 // add\n+ 2"),
        Ok(vec![
            SpannedToken::new(Token::Int(1), 0),
            SpannedToken::new(Token::Plus, 9),
            SpannedToken::new(Token::Int(2), 11),
            SpannedToken::new(Token::EoF, 12),
        ]),
    );
}

// invalid tokens

#[test]
fn invalid_character() {
    assert_eq!(
        lex("@"),
        Err(vec![SpannedLexError::new(LexError::InvalidChar('@'), 0)])
    );
}

#[test]
fn invalid_among_valid() {
    assert_eq!(
        lex("1 @ 2"),
        Err(vec![SpannedLexError::new(LexError::InvalidChar('@'), 2)])
    );
}

#[test]
fn multiple_invalid_chars() {
    assert_eq!(
        lex("~#$"),
        Err(vec![
            SpannedLexError::new(LexError::InvalidChar('~'), 0),
            SpannedLexError::new(LexError::InvalidChar('#'), 1),
            SpannedLexError::new(LexError::InvalidChar('$'), 2),
        ])
    );
}

// offset tracking

#[test]
fn offsets_no_whitespace() {
    assert_eq!(
        lex("1+2").unwrap(),
        vec![
            SpannedToken::new(Token::Int(1), 0),
            SpannedToken::new(Token::Plus, 1),
            SpannedToken::new(Token::Int(2), 2),
            SpannedToken::new(Token::EoF, 3),
        ]
    );
}

#[test]
fn offsets_with_spaces() {
    assert_eq!(
        lex("let x = 5").unwrap(),
        vec![
            SpannedToken::new(Token::Let, 0),
            SpannedToken::new(Token::Id("x"), 4),
            SpannedToken::new(Token::Assign, 6),
            SpannedToken::new(Token::Int(5), 8),
            SpannedToken::new(Token::EoF, 9),
        ]
    );
}

#[test]
fn offsets_with_newlines() {
    assert_eq!(
        lex("1\n+\n2").unwrap(),
        vec![
            SpannedToken::new(Token::Int(1), 0),
            SpannedToken::new(Token::Plus, 2),
            SpannedToken::new(Token::Int(2), 4),
            SpannedToken::new(Token::EoF, 5),
        ]
    );
}

#[test]
fn offset_after_comment() {
    assert_eq!(
        lex("// comment\n42").unwrap(),
        vec![
            SpannedToken::new(Token::Int(42), 11),
            SpannedToken::new(Token::EoF, 13),
        ]
    );
}

#[test]
fn eof_offset() {
    assert_eq!(
        lex("hi"),
        Ok(vec![
            SpannedToken::new(Token::Id("hi"), 0),
            SpannedToken::new(Token::EoF, 2),
        ]),
    );
}

// multi-token sequences

#[test]
fn let_binding() {
    assert_eq!(
        lex("let x = 42;"),
        Ok(vec![
            SpannedToken::new(Token::Let, 0),
            SpannedToken::new(Token::Id("x"), 4),
            SpannedToken::new(Token::Assign, 6),
            SpannedToken::new(Token::Int(42), 8),
            SpannedToken::new(Token::Semicolon, 10),
            SpannedToken::new(Token::EoF, 11),
        ]),
    );
}

#[test]
fn function_definition() {
    assert_eq!(
        lex("fn add(a: Int, b: Int) -> Int { return a + b; }"),
        Ok(vec![
            SpannedToken::new(Token::Fn, 0),
            SpannedToken::new(Token::Id("add"), 3),
            SpannedToken::new(Token::LParen, 6),
            SpannedToken::new(Token::Id("a"), 7),
            SpannedToken::new(Token::Colon, 8),
            SpannedToken::new(Token::IntType, 10),
            SpannedToken::new(Token::Comma, 13),
            SpannedToken::new(Token::Id("b"), 15),
            SpannedToken::new(Token::Colon, 16),
            SpannedToken::new(Token::IntType, 18),
            SpannedToken::new(Token::RParen, 21),
            SpannedToken::new(Token::Arrow, 23),
            SpannedToken::new(Token::IntType, 26),
            SpannedToken::new(Token::LBrace, 30),
            SpannedToken::new(Token::Return, 32),
            SpannedToken::new(Token::Id("a"), 39),
            SpannedToken::new(Token::Plus, 41),
            SpannedToken::new(Token::Id("b"), 43),
            SpannedToken::new(Token::Semicolon, 44),
            SpannedToken::new(Token::RBrace, 46),
            SpannedToken::new(Token::EoF, 47),
        ]),
    );
}

#[test]
fn if_then_else() {
    assert_eq!(
        lex("if x == 0 then 1 else 2"),
        Ok(vec![
            SpannedToken::new(Token::If, 0),
            SpannedToken::new(Token::Id("x"), 3),
            SpannedToken::new(Token::Eq, 5),
            SpannedToken::new(Token::Int(0), 8),
            SpannedToken::new(Token::Then, 10),
            SpannedToken::new(Token::Int(1), 15),
            SpannedToken::new(Token::Else, 17),
            SpannedToken::new(Token::Int(2), 22),
            SpannedToken::new(Token::EoF, 23),
        ]),
    );
}

#[test]
fn match_expression() {
    assert_eq!(
        lex("match x { 1 -> true };"),
        Ok(vec![
            SpannedToken::new(Token::Match, 0),
            SpannedToken::new(Token::Id("x"), 6),
            SpannedToken::new(Token::LBrace, 8),
            SpannedToken::new(Token::Int(1), 10),
            SpannedToken::new(Token::Arrow, 12),
            SpannedToken::new(Token::Bool(true), 15),
            SpannedToken::new(Token::RBrace, 20),
            SpannedToken::new(Token::Semicolon, 21),
            SpannedToken::new(Token::EoF, 22),
        ]),
    );
}

#[test]
fn pipe_operator_chain() {
    assert_eq!(
        lex("x |> f |> g"),
        Ok(vec![
            SpannedToken::new(Token::Id("x"), 0),
            SpannedToken::new(Token::Pipe, 2),
            SpannedToken::new(Token::Id("f"), 5),
            SpannedToken::new(Token::Pipe, 7),
            SpannedToken::new(Token::Id("g"), 10),
            SpannedToken::new(Token::EoF, 11),
        ]),
    );
}

#[test]
fn typedef_statement() {
    assert_eq!(
        lex("typedef MyList = Int;"),
        Ok(vec![
            SpannedToken::new(Token::Typedef, 0),
            SpannedToken::new(Token::TypeId("MyList"), 8),
            SpannedToken::new(Token::Assign, 15),
            SpannedToken::new(Token::IntType, 17),
            SpannedToken::new(Token::Semicolon, 20),
            SpannedToken::new(Token::EoF, 21),
        ]),
    );
}

#[test]
fn generic_typedef_statement() {
    assert_eq!(
        lex("typedef Option('a) { some(x: 'a), none() }"),
        Ok(vec![
            SpannedToken::new(Token::Typedef, 0),
            SpannedToken::new(Token::TypeId("Option"), 8),
            SpannedToken::new(Token::LParen, 14),
            SpannedToken::new(Token::TypeParam("'a"), 15),
            SpannedToken::new(Token::RParen, 17),
            SpannedToken::new(Token::LBrace, 19),
            SpannedToken::new(Token::Id("some"), 21),
            SpannedToken::new(Token::LParen, 25),
            SpannedToken::new(Token::Id("x"), 26),
            SpannedToken::new(Token::Colon, 27),
            SpannedToken::new(Token::TypeParam("'a"), 29),
            SpannedToken::new(Token::RParen, 31),
            SpannedToken::new(Token::Comma, 32),
            SpannedToken::new(Token::Id("none"), 34),
            SpannedToken::new(Token::LParen, 38),
            SpannedToken::new(Token::RParen, 39),
            SpannedToken::new(Token::RBrace, 41),
            SpannedToken::new(Token::EoF, 42),
        ]),
    );
}

#[test]
fn boolean_expression() {
    assert_eq!(
        lex("!a && b || c != d"),
        Ok(vec![
            SpannedToken::new(Token::Not, 0),
            SpannedToken::new(Token::Id("a"), 1),
            SpannedToken::new(Token::And, 3),
            SpannedToken::new(Token::Id("b"), 6),
            SpannedToken::new(Token::Or, 8),
            SpannedToken::new(Token::Id("c"), 11),
            SpannedToken::new(Token::NotEq, 13),
            SpannedToken::new(Token::Id("d"), 16),
            SpannedToken::new(Token::EoF, 17),
        ]),
    );
}

#[test]
fn comparison_operators() {
    assert_eq!(
        lex("a < b <= c > d >= e"),
        Ok(vec![
            SpannedToken::new(Token::Id("a"), 0),
            SpannedToken::new(Token::LessThan, 2),
            SpannedToken::new(Token::Id("b"), 4),
            SpannedToken::new(Token::LessThanOrEq, 6),
            SpannedToken::new(Token::Id("c"), 9),
            SpannedToken::new(Token::GreaterThan, 11),
            SpannedToken::new(Token::Id("d"), 13),
            SpannedToken::new(Token::GreaterThanOrEq, 15),
            SpannedToken::new(Token::Id("e"), 18),
            SpannedToken::new(Token::EoF, 19),
        ]),
    );
}

#[test]
fn float_comparison_operators() {
    assert_eq!(
        lex("a <. b <=. c >. d >=. e"),
        Ok(vec![
            SpannedToken::new(Token::Id("a"), 0),
            SpannedToken::new(Token::FLessThan, 2),
            SpannedToken::new(Token::Id("b"), 5),
            SpannedToken::new(Token::FLessThanOrEq, 7),
            SpannedToken::new(Token::Id("c"), 11),
            SpannedToken::new(Token::FGreaterThan, 13),
            SpannedToken::new(Token::Id("d"), 16),
            SpannedToken::new(Token::FGreaterThanOrEq, 18),
            SpannedToken::new(Token::Id("e"), 22),
            SpannedToken::new(Token::EoF, 23),
        ]),
    );
}

#[test]
fn arithmetic_expression() {
    assert_eq!(
        lex("a * b + c - d / e"),
        Ok(vec![
            SpannedToken::new(Token::Id("a"), 0),
            SpannedToken::new(Token::Times, 2),
            SpannedToken::new(Token::Id("b"), 4),
            SpannedToken::new(Token::Plus, 6),
            SpannedToken::new(Token::Id("c"), 8),
            SpannedToken::new(Token::Minus, 10),
            SpannedToken::new(Token::Id("d"), 12),
            SpannedToken::new(Token::Divide, 14),
            SpannedToken::new(Token::Id("e"), 16),
            SpannedToken::new(Token::EoF, 17),
        ]),
    );
}

#[test]
fn float_arithmetic_expression() {
    assert_eq!(
        lex("a +. b -. c *. d /. e"),
        Ok(vec![
            SpannedToken::new(Token::Id("a"), 0),
            SpannedToken::new(Token::FPlus, 2),
            SpannedToken::new(Token::Id("b"), 5),
            SpannedToken::new(Token::FMinus, 7),
            SpannedToken::new(Token::Id("c"), 10),
            SpannedToken::new(Token::FTimes, 12),
            SpannedToken::new(Token::Id("d"), 15),
            SpannedToken::new(Token::FDivide, 17),
            SpannedToken::new(Token::Id("e"), 20),
            SpannedToken::new(Token::EoF, 21),
        ]),
    );
}

#[test]
fn mixed_int_and_float_operators() {
    assert_eq!(
        lex("x + y +. z - a -. b"),
        Ok(vec![
            SpannedToken::new(Token::Id("x"), 0),
            SpannedToken::new(Token::Plus, 2),
            SpannedToken::new(Token::Id("y"), 4),
            SpannedToken::new(Token::FPlus, 6),
            SpannedToken::new(Token::Id("z"), 9),
            SpannedToken::new(Token::Minus, 11),
            SpannedToken::new(Token::Id("a"), 13),
            SpannedToken::new(Token::FMinus, 15),
            SpannedToken::new(Token::Id("b"), 18),
            SpannedToken::new(Token::EoF, 19),
        ]),
    );
}

// edge cases

#[test]
fn adjacent_operators() {
    // "!=" is NotEq, but "! =" is Not then Assign
    assert_eq!(
        lex("! ="),
        Ok(vec![
            SpannedToken::new(Token::Not, 0),
            SpannedToken::new(Token::Assign, 2),
            SpannedToken::new(Token::EoF, 3),
        ]),
    );
}

#[test]
fn negative_number_is_minus_then_int() {
    // the lexer doesn't produce negative literals; "-5" is minus then int(5)
    assert_eq!(
        lex("-5"),
        Ok(vec![
            SpannedToken::new(Token::Minus, 0),
            SpannedToken::new(Token::Int(5), 1),
            SpannedToken::new(Token::EoF, 2),
        ]),
    );
}

#[test]
fn underscore_only_identifier() {
    // a lone underscore starts with non-uppercase, non-digit, non-operator
    // it's not lowercase either, so it should be invalid
    assert_eq!(
        lex("_"),
        Err(vec![SpannedLexError::new(LexError::InvalidChar('_'), 0)])
    );
}

#[test]
fn echo_and_read_keywords() {
    assert_eq!(
        lex("echo read"),
        Ok(vec![
            SpannedToken::new(Token::Echo, 0),
            SpannedToken::new(Token::Read, 5),
            SpannedToken::new(Token::EoF, 9),
        ]),
    );
}

#[test]
fn divide_not_confused_with_comment() {
    // "/" alone is Divide, "//" starts a comment
    assert_eq!(
        lex("a / b"),
        Ok(vec![
            SpannedToken::new(Token::Id("a"), 0),
            SpannedToken::new(Token::Divide, 2),
            SpannedToken::new(Token::Id("b"), 4),
            SpannedToken::new(Token::EoF, 5),
        ]),
    );
}

#[test]
fn multiline_program() {
    let program = "\
fn main() -> Int {
    let x = 10;
    let y = 20;
    return x + y;
}";
    assert_eq!(
        lex(program),
        Ok(vec![
            SpannedToken::new(Token::Fn, 0),
            SpannedToken::new(Token::Id("main"), 3),
            SpannedToken::new(Token::LParen, 7),
            SpannedToken::new(Token::RParen, 8),
            SpannedToken::new(Token::Arrow, 10),
            SpannedToken::new(Token::IntType, 13),
            SpannedToken::new(Token::LBrace, 17),
            SpannedToken::new(Token::Let, 23),
            SpannedToken::new(Token::Id("x"), 27),
            SpannedToken::new(Token::Assign, 29),
            SpannedToken::new(Token::Int(10), 31),
            SpannedToken::new(Token::Semicolon, 33),
            SpannedToken::new(Token::Let, 39),
            SpannedToken::new(Token::Id("y"), 43),
            SpannedToken::new(Token::Assign, 45),
            SpannedToken::new(Token::Int(20), 47),
            SpannedToken::new(Token::Semicolon, 49),
            SpannedToken::new(Token::Return, 55),
            SpannedToken::new(Token::Id("x"), 62),
            SpannedToken::new(Token::Plus, 64),
            SpannedToken::new(Token::Id("y"), 66),
            SpannedToken::new(Token::Semicolon, 67),
            SpannedToken::new(Token::RBrace, 69),
            SpannedToken::new(Token::EoF, 70),
        ]),
    );
}

#[test]
fn lex_includes_eof() {
    assert_eq!(
        lex("42"),
        Ok(vec![
            SpannedToken::new(Token::Int(42), 0),
            SpannedToken::new(Token::EoF, "42".len()),
        ]),
    );
}
