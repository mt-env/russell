use super::lex;
use super::token::{LexError, Token};

/// Helper: lex a string and return every Token variant, including EoF.
fn tokens(input: &str) -> Vec<Token<'_>> {
    lex(input)
        .expect("expected input to lex successfully")
        .into_iter()
        .map(|st| st.node)
        .collect()
}

/// Helper: lex a string and return every (Token, offset) pair, including EoF.
fn tokens_with_offsets(input: &str) -> Vec<(Token<'_>, usize)> {
    lex(input)
        .expect("expected input to lex successfully")
        .into_iter()
        .map(|st| (st.node, st.offset))
        .collect()
}

/// helper: assert that a single-token input produces the expected token
fn assert_single(input: &str, expected: Token<'_>) {
    assert_eq!(tokens(input), vec![expected, Token::EoF]);
}

// empty/EoF

#[test]
fn empty_input() {
    let result = lex("").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].node, Token::EoF);
    assert_eq!(result[0].offset, 0);
}

#[test]
fn whitespace_only() {
    let result = lex("   \t\n  ").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].node, Token::EoF);
}

// integer literals

#[test]
fn single_digit_int() {
    assert_eq!(tokens("5"), vec![Token::Int(5), Token::EoF]);
}

#[test]
fn multi_digit_int() {
    assert_eq!(tokens("12345"), vec![Token::Int(12345), Token::EoF]);
}

#[test]
fn zero() {
    assert_eq!(tokens("0"), vec![Token::Int(0), Token::EoF]);
}

// float literals

#[test]
fn simple_float() {
    assert_eq!(tokens("3.14"), vec![Token::Float(3.14), Token::EoF]);
}

#[test]
fn float_leading_zero() {
    assert_eq!(tokens("0.5"), vec![Token::Float(0.5), Token::EoF]);
}

#[test]
fn float_trailing_dot() {
    assert_eq!(tokens("1."), vec![Token::Float(1.0), Token::EoF]);
}

#[test]
fn float_trailing_dot_before_identifier() {
    assert_eq!(
        tokens("1.foo"),
        vec![Token::Float(1.0), Token::Id("foo"), Token::EoF]
    );
}

#[test]
fn number_with_two_dots() {
    let errors = lex("1.2.3").unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].node, LexError::InvalidFloat("1.2.3"));
    assert_eq!(errors[0].offset, 0);
}

// boolean literals

#[test]
fn true_literal() {
    assert_eq!(tokens("true"), vec![Token::Bool(true), Token::EoF]);
}

#[test]
fn false_literal() {
    assert_eq!(tokens("false"), vec![Token::Bool(false), Token::EoF]);
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
        assert_single(input, expected);
    }
}

#[test]
fn keyword_prefix_is_identifier() {
    // "letters" starts with "let" but should not be a keyword
    assert_eq!(tokens("letters"), vec![Token::Id("letters"), Token::EoF]);
}

#[test]
fn keyword_with_suffix_is_identifier() {
    assert_eq!(tokens("iffoo"), vec![Token::Id("iffoo"), Token::EoF]);
    assert_eq!(
        tokens("return_value"),
        vec![Token::Id("return_value"), Token::EoF]
    );
}

// identifiers

#[test]
fn simple_identifier() {
    assert_eq!(tokens("foo"), vec![Token::Id("foo"), Token::EoF]);
}

#[test]
fn identifier_with_underscores() {
    assert_eq!(
        tokens("my_var_name"),
        vec![Token::Id("my_var_name"), Token::EoF]
    );
}

#[test]
fn identifier_with_digits() {
    assert_eq!(tokens("x2"), vec![Token::Id("x2"), Token::EoF]);
}

// type identifiers

#[test]
fn builtin_type_keywords() {
    assert_single("Int", Token::IntType);
    assert_single("Float", Token::FloatType);
    assert_single("Bool", Token::BoolType);
}

#[test]
fn custom_type_identifier() {
    assert_eq!(tokens("MyType"), vec![Token::TypeId("MyType"), Token::EoF]);
}

#[test]
fn type_id_allows_digits_and_underscores() {
    assert_eq!(
        tokens("Vec2_Type"),
        vec![Token::TypeId("Vec2_Type"), Token::EoF]
    );
}

#[test]
fn type_param_single_letter() {
    assert_eq!(tokens("'a"), vec![Token::TypeParam("'a"), Token::EoF]);
}

#[test]
fn type_param_stops_at_non_alpha() {
    assert_eq!(
        tokens("'abc1"),
        vec![Token::TypeParam("'abc"), Token::Int(1), Token::EoF]
    );
}

#[test]
fn invalid_type_param_without_name() {
    let errors = lex("'").unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].node, LexError::InvalidChar('\''));
    assert_eq!(errors[0].offset, 0);
}

#[test]
fn invalid_type_param_with_uppercase() {
    let errors = lex("'Abc").unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].node, LexError::InvalidChar('\''));
    assert_eq!(errors[0].offset, 0);
}

#[test]
fn invalid_type_param_without_lowercase() {
    let errors = lex("'_x").unwrap_err();
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].node, LexError::InvalidChar('\''));
    assert_eq!(errors[1].node, LexError::InvalidChar('_'));
    assert_eq!(errors[0].offset, 0);
    assert_eq!(errors[1].offset, 1);
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
        assert_single(input, expected);
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
        assert_single(input, expected);
    }
}

#[test]
fn two_char_op_preferred_over_one_char() {
    // "!=" should be NotEq, not Not followed by Assign
    assert_eq!(tokens("!="), vec![Token::NotEq, Token::EoF]);

    // "<=" should be LessThanOrEq, not LessThan followed by Assign
    assert_eq!(tokens("<="), vec![Token::LessThanOrEq, Token::EoF]);
}

#[test]
fn three_char_op_preferred_over_shorter_prefixes() {
    assert_eq!(tokens("<=."), vec![Token::FLessThanOrEq, Token::EoF]);
    assert_eq!(tokens(">=."), vec![Token::FGreaterThanOrEq, Token::EoF]);
}

// whitespace handling

#[test]
fn spaces_between_tokens() {
    assert_eq!(
        tokens("1 + 2"),
        vec![Token::Int(1), Token::Plus, Token::Int(2), Token::EoF]
    );
}

#[test]
fn tabs_and_newlines() {
    assert_eq!(
        tokens("let\n\tx\t=\n5"),
        vec![
            Token::Let,
            Token::Id("x"),
            Token::Assign,
            Token::Int(5),
            Token::EoF
        ]
    );
}

#[test]
fn no_whitespace_between_tokens() {
    assert_eq!(
        tokens("1+2"),
        vec![Token::Int(1), Token::Plus, Token::Int(2), Token::EoF]
    );
}

// comments

#[test]
fn line_comment_skipped() {
    assert_eq!(
        tokens("// this is a comment\n42"),
        vec![Token::Int(42), Token::EoF]
    );
}

#[test]
fn comment_at_end_of_line() {
    assert_eq!(
        tokens("42 // trailing comment"),
        vec![Token::Int(42), Token::EoF]
    );
}

#[test]
fn multiple_comment_lines() {
    assert_eq!(
        tokens("// first\n// second\n// third\n7"),
        vec![Token::Int(7), Token::EoF]
    );
}

#[test]
fn comment_only_no_newline() {
    let result = lex("// just a comment").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].node, Token::EoF);
}

#[test]
fn comment_between_tokens() {
    assert_eq!(
        tokens("1 // add\n+ 2"),
        vec![Token::Int(1), Token::Plus, Token::Int(2), Token::EoF]
    );
}

// invalid tokens

#[test]
fn invalid_character() {
    let errors = lex("@").unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].node, LexError::InvalidChar('@'));
    assert_eq!(errors[0].offset, 0);
}

#[test]
fn invalid_among_valid() {
    let errors = lex("1 @ 2").unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].node, LexError::InvalidChar('@'));
    assert_eq!(errors[0].offset, 2);
}

#[test]
fn multiple_invalid_chars() {
    let errors = lex("~#$").unwrap_err();
    assert_eq!(errors.len(), 3);
    assert_eq!(errors[0].node, LexError::InvalidChar('~'));
    assert_eq!(errors[1].node, LexError::InvalidChar('#'));
    assert_eq!(errors[2].node, LexError::InvalidChar('$'));
    assert_eq!(errors[0].offset, 0);
    assert_eq!(errors[1].offset, 1);
    assert_eq!(errors[2].offset, 2);
}

// offset tracking

#[test]
fn offsets_no_whitespace() {
    assert_eq!(
        tokens_with_offsets("1+2"),
        vec![
            (Token::Int(1), 0),
            (Token::Plus, 1),
            (Token::Int(2), 2),
            (Token::EoF, 3),
        ]
    );
}

#[test]
fn offsets_with_spaces() {
    assert_eq!(
        tokens_with_offsets("let x = 5"),
        vec![
            (Token::Let, 0),
            (Token::Id("x"), 4),
            (Token::Assign, 6),
            (Token::Int(5), 8),
            (Token::EoF, 9),
        ]
    );
}

#[test]
fn offsets_with_newlines() {
    assert_eq!(
        tokens_with_offsets("1\n+\n2"),
        vec![
            (Token::Int(1), 0),
            (Token::Plus, 2),
            (Token::Int(2), 4),
            (Token::EoF, 5),
        ]
    );
}

#[test]
fn offset_after_comment() {
    assert_eq!(
        tokens_with_offsets("// comment\n42"),
        vec![(Token::Int(42), 11), (Token::EoF, 13)]
    );
}

#[test]
fn eof_offset() {
    let result = lex("hi").unwrap();
    let eof = result.last().unwrap();
    assert_eq!(eof.node, Token::EoF);
    assert_eq!(eof.offset, 2);
}

// multi-token sequences

#[test]
fn let_binding() {
    assert_eq!(
        tokens("let x = 42;"),
        vec![
            Token::Let,
            Token::Id("x"),
            Token::Assign,
            Token::Int(42),
            Token::Semicolon,
            Token::EoF,
        ]
    );
}

#[test]
fn function_definition() {
    let toks = tokens("fn add(a: Int, b: Int) -> Int { return a + b; }");
    assert_eq!(
        toks,
        vec![
            Token::Fn,
            Token::Id("add"),
            Token::LParen,
            Token::Id("a"),
            Token::Colon,
            Token::IntType,
            Token::Comma,
            Token::Id("b"),
            Token::Colon,
            Token::IntType,
            Token::RParen,
            Token::Arrow,
            Token::IntType,
            Token::LBrace,
            Token::Return,
            Token::Id("a"),
            Token::Plus,
            Token::Id("b"),
            Token::Semicolon,
            Token::RBrace,
            Token::EoF,
        ]
    );
}

#[test]
fn if_then_else() {
    let toks = tokens("if x == 0 then 1 else 2");
    assert_eq!(
        toks,
        vec![
            Token::If,
            Token::Id("x"),
            Token::Eq,
            Token::Int(0),
            Token::Then,
            Token::Int(1),
            Token::Else,
            Token::Int(2),
            Token::EoF,
        ]
    );
}

#[test]
fn match_expression() {
    let toks = tokens("match x { 1 -> true };");
    assert_eq!(
        toks,
        vec![
            Token::Match,
            Token::Id("x"),
            Token::LBrace,
            Token::Int(1),
            Token::Arrow,
            Token::Bool(true),
            Token::RBrace,
            Token::Semicolon,
            Token::EoF,
        ]
    );
}

#[test]
fn pipe_operator_chain() {
    assert_eq!(
        tokens("x |> f |> g"),
        vec![
            Token::Id("x"),
            Token::Pipe,
            Token::Id("f"),
            Token::Pipe,
            Token::Id("g"),
            Token::EoF,
        ]
    );
}

#[test]
fn typedef_statement() {
    assert_eq!(
        tokens("typedef MyList = Int;"),
        vec![
            Token::Typedef,
            Token::TypeId("MyList"),
            Token::Assign,
            Token::IntType,
            Token::Semicolon,
            Token::EoF,
        ]
    );
}

#[test]
fn generic_typedef_statement() {
    let toks = tokens("typedef Option('a) { some(x: 'a), none() }");
    assert_eq!(
        toks,
        vec![
            Token::Typedef,
            Token::TypeId("Option"),
            Token::LParen,
            Token::TypeParam("'a"),
            Token::RParen,
            Token::LBrace,
            Token::Id("some"),
            Token::LParen,
            Token::Id("x"),
            Token::Colon,
            Token::TypeParam("'a"),
            Token::RParen,
            Token::Comma,
            Token::Id("none"),
            Token::LParen,
            Token::RParen,
            Token::RBrace,
            Token::EoF,
        ]
    );
}

#[test]
fn boolean_expression() {
    assert_eq!(
        tokens("!a && b || c != d"),
        vec![
            Token::Not,
            Token::Id("a"),
            Token::And,
            Token::Id("b"),
            Token::Or,
            Token::Id("c"),
            Token::NotEq,
            Token::Id("d"),
            Token::EoF,
        ]
    );
}

#[test]
fn comparison_operators() {
    assert_eq!(
        tokens("a < b <= c > d >= e"),
        vec![
            Token::Id("a"),
            Token::LessThan,
            Token::Id("b"),
            Token::LessThanOrEq,
            Token::Id("c"),
            Token::GreaterThan,
            Token::Id("d"),
            Token::GreaterThanOrEq,
            Token::Id("e"),
            Token::EoF,
        ]
    );
}

#[test]
fn float_comparison_operators() {
    assert_eq!(
        tokens("a <. b <=. c >. d >=. e"),
        vec![
            Token::Id("a"),
            Token::FLessThan,
            Token::Id("b"),
            Token::FLessThanOrEq,
            Token::Id("c"),
            Token::FGreaterThan,
            Token::Id("d"),
            Token::FGreaterThanOrEq,
            Token::Id("e"),
            Token::EoF,
        ]
    );
}

#[test]
fn arithmetic_expression() {
    assert_eq!(
        tokens("a * b + c - d / e"),
        vec![
            Token::Id("a"),
            Token::Times,
            Token::Id("b"),
            Token::Plus,
            Token::Id("c"),
            Token::Minus,
            Token::Id("d"),
            Token::Divide,
            Token::Id("e"),
            Token::EoF,
        ]
    );
}

#[test]
fn float_arithmetic_expression() {
    assert_eq!(
        tokens("a +. b -. c *. d /. e"),
        vec![
            Token::Id("a"),
            Token::FPlus,
            Token::Id("b"),
            Token::FMinus,
            Token::Id("c"),
            Token::FTimes,
            Token::Id("d"),
            Token::FDivide,
            Token::Id("e"),
            Token::EoF,
        ]
    );
}

#[test]
fn mixed_int_and_float_operators() {
    assert_eq!(
        tokens("x + y +. z - a -. b"),
        vec![
            Token::Id("x"),
            Token::Plus,
            Token::Id("y"),
            Token::FPlus,
            Token::Id("z"),
            Token::Minus,
            Token::Id("a"),
            Token::FMinus,
            Token::Id("b"),
            Token::EoF,
        ]
    );
}

// edge cases

#[test]
fn adjacent_operators() {
    // "!=" is NotEq, but "! =" is Not then Assign
    assert_eq!(tokens("! ="), vec![Token::Not, Token::Assign, Token::EoF]);
}

#[test]
fn negative_number_is_minus_then_int() {
    // the lexer doesn't produce negative literals; "-5" is minus then int(5)
    assert_eq!(tokens("-5"), vec![Token::Minus, Token::Int(5), Token::EoF]);
}

#[test]
fn underscore_only_identifier() {
    // a lone underscore starts with non-uppercase, non-digit, non-operator
    // it's not lowercase either, so it should be invalid
    let errors = lex("_").unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].node, LexError::InvalidChar('_'));
    assert_eq!(errors[0].offset, 0);
}

#[test]
fn echo_and_read_keywords() {
    assert_eq!(
        tokens("echo read"),
        vec![Token::Echo, Token::Read, Token::EoF]
    );
}

#[test]
fn divide_not_confused_with_comment() {
    // "/" alone is Divide, "//" starts a comment
    assert_eq!(
        tokens("a / b"),
        vec![Token::Id("a"), Token::Divide, Token::Id("b"), Token::EoF]
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
        tokens(program),
        vec![
            Token::Fn,
            Token::Id("main"),
            Token::LParen,
            Token::RParen,
            Token::Arrow,
            Token::IntType,
            Token::LBrace,
            Token::Let,
            Token::Id("x"),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Id("y"),
            Token::Assign,
            Token::Int(20),
            Token::Semicolon,
            Token::Return,
            Token::Id("x"),
            Token::Plus,
            Token::Id("y"),
            Token::Semicolon,
            Token::RBrace,
            Token::EoF,
        ]
    );
}

#[test]
fn lex_includes_eof() {
    let result = lex("42").unwrap();
    assert_eq!(result[0].node, Token::Int(42));
    assert_eq!(result[1].node, Token::EoF);
}
