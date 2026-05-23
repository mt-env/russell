use super::lex;
use super::token::{Token, TokenKind};

/// helper: lex a string and return just the Token variants (no offsets), excluding EoF
fn tokens(input: &str) -> Vec<Token> {
    lex(input)
        .into_iter()
        .filter(|st| !matches!(st.token, Token::EoF))
        .map(|st| st.token)
        .collect()
}

/// helper: lex a string and return (Token, offset) pairs, excluding EoF
fn tokens_with_offsets(input: &str) -> Vec<(Token, usize)> {
    lex(input)
        .into_iter()
        .filter(|st| !matches!(st.token, Token::EoF))
        .map(|st| (st.token, st.offset))
        .collect()
}

/// helper: assert that a single-token input produces the expected token kind
fn assert_single(input: &str, expected: TokenKind) {
    let toks = tokens(input);
    assert_eq!(toks.len(), 1, "expected 1 token for {:?}, got {:?}", input, toks);
    assert_eq!(toks[0].kind(), expected);
}

// empty/EoF

#[test]
fn empty_input() {
    let result = lex("");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].kind(), TokenKind::EoF);
    assert_eq!(result[0].offset, 0);
}

#[test]
fn whitespace_only() {
    let result = lex("   \t\n  ");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].kind(), TokenKind::EoF);
}

// integer literals

#[test]
fn single_digit_int() {
    let toks = tokens("5");
    assert!(matches!(toks[0], Token::Int(5)));
}

#[test]
fn multi_digit_int() {
    let toks = tokens("12345");
    assert!(matches!(toks[0], Token::Int(12345)));
}

#[test]
fn zero() {
    let toks = tokens("0");
    assert!(matches!(toks[0], Token::Int(0)));
}

// float literals

#[test]
fn simple_float() {
    let toks = tokens("3.14");
    match &toks[0] {
        Token::Float(f) => assert!((*f - 3.14).abs() < 1e-10),
        other => panic!("expected Float, got {:?}", other),
    }
}

#[test]
fn float_leading_zero() {
    let toks = tokens("0.5");
    match &toks[0] {
        Token::Float(f) => assert!((*f - 0.5).abs() < 1e-10),
        other => panic!("expected Float, got {:?}", other),
    }
}

#[test]
fn float_trailing_dot() {
    // "1." should lex as a float (dot is consumed greedily)
    let toks = tokens("1.");
    assert_eq!(toks[0].kind(), TokenKind::Float);
}

#[test]
fn number_with_two_dots() {
    // "1.2.3" — first dot is consumed, second dot is not a digit so it stops
    // should produce Float(1.2) then Invalid('.') then Int(3)
    let toks = tokens("1.2.3");
    assert_eq!(toks[0].kind(), TokenKind::Float);
    assert_eq!(toks[1].kind(), TokenKind::Invalid);
    assert_eq!(toks[2].kind(), TokenKind::Int);
}

// boolean literals

#[test]
fn true_literal() {
    let toks = tokens("true");
    assert!(matches!(toks[0], Token::Bool(true)));
}

#[test]
fn false_literal() {
    let toks = tokens("false");
    assert!(matches!(toks[0], Token::Bool(false)));
}

// keywords

#[test]
fn all_keywords() {
    let cases = [
        ("echo", TokenKind::Echo),
        ("else", TokenKind::Else),
        ("fn", TokenKind::Fn),
        ("if", TokenKind::If),
        ("let", TokenKind::Let),
        ("match", TokenKind::Match),
        ("read", TokenKind::Read),
        ("return", TokenKind::Return),
        ("then", TokenKind::Then),
        ("typedef", TokenKind::Typedef),
    ];
    for (input, expected) in cases {
        assert_single(input, expected);
    }
}

#[test]
fn keyword_prefix_is_identifier() {
    // "letters" starts with "let" but should not be a keyword
    let toks = tokens("letters");
    assert_eq!(toks[0].kind(), TokenKind::Id);
    match &toks[0] {
        Token::Id(s) => assert_eq!(*s, "letters"),
        other => panic!("expected Id, got {:?}", other),
    }
}

#[test]
fn keyword_with_suffix_is_identifier() {
    let toks = tokens("iffoo");
    assert_eq!(toks[0].kind(), TokenKind::Id);

    let toks = tokens("return_value");
    assert_eq!(toks[0].kind(), TokenKind::Id);
}

// identifiers

#[test]
fn simple_identifier() {
    let toks = tokens("foo");
    match &toks[0] {
        Token::Id(s) => assert_eq!(*s, "foo"),
        other => panic!("expected Id, got {:?}", other),
    }
}

#[test]
fn identifier_with_underscores() {
    let toks = tokens("my_var_name");
    match &toks[0] {
        Token::Id(s) => assert_eq!(*s, "my_var_name"),
        other => panic!("expected Id, got {:?}", other),
    }
}

#[test]
fn identifier_with_digits() {
    let toks = tokens("x2");
    match &toks[0] {
        Token::Id(s) => assert_eq!(*s, "x2"),
        other => panic!("expected Id, got {:?}", other),
    }
}

// type identifiers

#[test]
fn builtin_type_keywords() {
    assert_single("Int", TokenKind::IntType);
    assert_single("Float", TokenKind::FloatType);
    assert_single("Bool", TokenKind::BoolType);
}

#[test]
fn custom_type_identifier() {
    let toks = tokens("MyType");
    match &toks[0] {
        Token::TypeId(s) => assert_eq!(*s, "MyType"),
        other => panic!("expected TypeId, got {:?}", other),
    }
}

#[test]
fn type_id_stops_at_non_alpha() {
    // type identifiers only consume alphabetic chars, not digits or underscores
    let toks = tokens("Vec2");
    match &toks[0] {
        Token::TypeId(s) => assert_eq!(*s, "Vec"),
        other => panic!("expected TypeId, got {:?}", other),
    }
    assert!(matches!(toks[1], Token::Int(2)));
}

// ─── One-character operators / punctuation ──────────────────────────────────

#[test]
fn single_char_operators() {
    let cases = [
        ("=", TokenKind::Assign),
        ("!", TokenKind::Not),
        ("(", TokenKind::LParen),
        (")", TokenKind::RParen),
        ("*", TokenKind::Times),
        ("+", TokenKind::Plus),
        (",", TokenKind::Comma),
        ("-", TokenKind::Minus),
        ("/", TokenKind::Divide),
        (":", TokenKind::Colon),
        (";", TokenKind::Semicolon),
        ("<", TokenKind::LessThan),
        (">", TokenKind::GreaterThan),
        ("{", TokenKind::LBrace),
        ("}", TokenKind::RBrace),
    ];
    for (input, expected) in cases {
        assert_single(input, expected);
    }
}

// two-character operators

#[test]
fn two_char_operators() {
    let cases = [
        ("!=", TokenKind::NotEq),
        ("&&", TokenKind::And),
        ("->", TokenKind::Arrow),
        ("<=", TokenKind::LessThanOrEq),
        ("==", TokenKind::Eq),
        (">=", TokenKind::GreaterThanOrEq),
        ("|>", TokenKind::Pipe),
        ("||", TokenKind::Or),
    ];
    for (input, expected) in cases {
        assert_single(input, expected);
    }
}

#[test]
fn two_char_op_preferred_over_one_char() {
    // "!=" should be NotEq, not Not followed by Assign
    let toks = tokens("!=");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].kind(), TokenKind::NotEq);

    // "<=" should be LessThanOrEq, not LessThan followed by Assign
    let toks = tokens("<=");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].kind(), TokenKind::LessThanOrEq);
}

// whitespace handling

#[test]
fn spaces_between_tokens() {
    let toks = tokens("1 + 2");
    assert_eq!(toks.len(), 3);
    assert!(matches!(toks[0], Token::Int(1)));
    assert_eq!(toks[1].kind(), TokenKind::Plus);
    assert!(matches!(toks[2], Token::Int(2)));
}

#[test]
fn tabs_and_newlines() {
    let toks = tokens("let\n\tx\t=\n5");
    assert_eq!(toks.len(), 4);
    assert_eq!(toks[0].kind(), TokenKind::Let);
    assert_eq!(toks[1].kind(), TokenKind::Id);
    assert_eq!(toks[2].kind(), TokenKind::Assign);
    assert!(matches!(toks[3], Token::Int(5)));
}

#[test]
fn no_whitespace_between_tokens() {
    let toks = tokens("1+2");
    assert_eq!(toks.len(), 3);
    assert!(matches!(toks[0], Token::Int(1)));
    assert_eq!(toks[1].kind(), TokenKind::Plus);
    assert!(matches!(toks[2], Token::Int(2)));
}

// comments

#[test]
fn line_comment_skipped() {
    let toks = tokens("// this is a comment\n42");
    assert_eq!(toks.len(), 1);
    assert!(matches!(toks[0], Token::Int(42)));
}

#[test]
fn comment_at_end_of_line() {
    let toks = tokens("42 // trailing comment");
    assert_eq!(toks.len(), 1);
    assert!(matches!(toks[0], Token::Int(42)));
}

#[test]
fn multiple_comment_lines() {
    let toks = tokens("// first\n// second\n// third\n7");
    assert_eq!(toks.len(), 1);
    assert!(matches!(toks[0], Token::Int(7)));
}

#[test]
fn comment_only_no_newline() {
    let result = lex("// just a comment");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].kind(), TokenKind::EoF);
}

#[test]
fn comment_between_tokens() {
    let toks = tokens("1 // add\n+ 2");
    assert_eq!(toks.len(), 3);
    assert!(matches!(toks[0], Token::Int(1)));
    assert_eq!(toks[1].kind(), TokenKind::Plus);
    assert!(matches!(toks[2], Token::Int(2)));
}

// invalid tokens

#[test]
fn invalid_character() {
    let toks = tokens("@");
    assert_eq!(toks.len(), 1);
    match &toks[0] {
        Token::Invalid(c) => assert_eq!(*c, '@'),
        other => panic!("expected Invalid('@'), got {:?}", other),
    }
}

#[test]
fn invalid_among_valid() {
    let toks = tokens("1 @ 2");
    assert_eq!(toks.len(), 3);
    assert!(matches!(toks[0], Token::Int(1)));
    assert!(matches!(toks[1], Token::Invalid('@')));
    assert!(matches!(toks[2], Token::Int(2)));
}

#[test]
fn multiple_invalid_chars() {
    let toks = tokens("~#$");
    assert_eq!(toks.len(), 3);
    assert!(matches!(toks[0], Token::Invalid('~')));
    assert!(matches!(toks[1], Token::Invalid('#')));
    assert!(matches!(toks[2], Token::Invalid('$')));
}

// offset tracking

#[test]
fn offsets_no_whitespace() {
    // "1+2" — offsets: 0, 1, 2
    let toks = tokens_with_offsets("1+2");
    assert_eq!(toks[0].1, 0);
    assert_eq!(toks[1].1, 1);
    assert_eq!(toks[2].1, 2);
}

#[test]
fn offsets_with_spaces() {
    // "let x = 5" — offsets: 0, 4, 6, 8
    let toks = tokens_with_offsets("let x = 5");
    assert_eq!(toks[0].1, 0); // "let"
    assert_eq!(toks[1].1, 4); // "x"
    assert_eq!(toks[2].1, 6); // "="
    assert_eq!(toks[3].1, 8); // "5"
}

#[test]
fn offsets_with_newlines() {
    let toks = tokens_with_offsets("1\n+\n2");
    assert_eq!(toks[0].1, 0); // "1"
    assert_eq!(toks[1].1, 2); // "+"
    assert_eq!(toks[2].1, 4); // "2"
}

#[test]
fn offset_after_comment() {
    let input = "// comment\n42";
    let toks = tokens_with_offsets(input);
    assert_eq!(toks[0].1, 11); // "42" starts after "// comment\n"
}

#[test]
fn eof_offset() {
    let result = lex("hi");
    let eof = result.last().unwrap();
    assert_eq!(eof.kind(), TokenKind::EoF);
    assert_eq!(eof.offset, 2);
}

// multi-token sequences

#[test]
fn let_binding() {
    let toks = tokens("let x = 42;");
    assert_eq!(toks.len(), 5);
    assert_eq!(toks[0].kind(), TokenKind::Let);
    assert_eq!(toks[1].kind(), TokenKind::Id);
    assert_eq!(toks[2].kind(), TokenKind::Assign);
    assert!(matches!(toks[3], Token::Int(42)));
    assert_eq!(toks[4].kind(), TokenKind::Semicolon);
}

#[test]
fn function_definition() {
    let toks = tokens("fn add(a: Int, b: Int) -> Int { return a + b; }");
    let kinds: Vec<TokenKind> = toks.iter().map(|t| t.kind()).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::Fn,
            TokenKind::Id, // add
            TokenKind::LParen,
            TokenKind::Id, // a
            TokenKind::Colon,
            TokenKind::IntType,
            TokenKind::Comma,
            TokenKind::Id, // b
            TokenKind::Colon,
            TokenKind::IntType,
            TokenKind::RParen,
            TokenKind::Arrow,
            TokenKind::IntType,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Id, // a
            TokenKind::Plus,
            TokenKind::Id, // b
            TokenKind::Semicolon,
            TokenKind::RBrace,
        ]
    );
}

#[test]
fn if_then_else() {
    let toks = tokens("if x == 0 then 1 else 2");
    let kinds: Vec<TokenKind> = toks.iter().map(|t| t.kind()).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::If,
            TokenKind::Id,
            TokenKind::Eq,
            TokenKind::Int,
            TokenKind::Then,
            TokenKind::Int,
            TokenKind::Else,
            TokenKind::Int,
        ]
    );
}

#[test]
fn match_expression() {
    let toks = tokens("match x { 1 -> true };");
    let kinds: Vec<TokenKind> = toks.iter().map(|t| t.kind()).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::Match,
            TokenKind::Id,
            TokenKind::LBrace,
            TokenKind::Int,
            TokenKind::Arrow,
            TokenKind::Bool,
            TokenKind::RBrace,
            TokenKind::Semicolon,
        ]
    );
}

#[test]
fn pipe_operator_chain() {
    let toks = tokens("x |> f |> g");
    let kinds: Vec<TokenKind> = toks.iter().map(|t| t.kind()).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::Id,
            TokenKind::Pipe,
            TokenKind::Id,
            TokenKind::Pipe,
            TokenKind::Id,
        ]
    );
}

#[test]
fn typedef_statement() {
    let toks = tokens("typedef MyList = Int;");
    let kinds: Vec<TokenKind> = toks.iter().map(|t| t.kind()).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::Typedef,
            TokenKind::TypeId,
            TokenKind::Assign,
            TokenKind::IntType,
            TokenKind::Semicolon,
        ]
    );
}

#[test]
fn boolean_expression() {
    let toks = tokens("!a && b || c != d");
    let kinds: Vec<TokenKind> = toks.iter().map(|t| t.kind()).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::Not,
            TokenKind::Id,
            TokenKind::And,
            TokenKind::Id,
            TokenKind::Or,
            TokenKind::Id,
            TokenKind::NotEq,
            TokenKind::Id,
        ]
    );
}

#[test]
fn comparison_operators() {
    let toks = tokens("a < b <= c > d >= e");
    let kinds: Vec<TokenKind> = toks.iter().map(|t| t.kind()).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::Id,
            TokenKind::LessThan,
            TokenKind::Id,
            TokenKind::LessThanOrEq,
            TokenKind::Id,
            TokenKind::GreaterThan,
            TokenKind::Id,
            TokenKind::GreaterThanOrEq,
            TokenKind::Id,
        ]
    );
}

#[test]
fn arithmetic_expression() {
    let toks = tokens("a * b + c - d / e");
    let kinds: Vec<TokenKind> = toks.iter().map(|t| t.kind()).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::Id,
            TokenKind::Times,
            TokenKind::Id,
            TokenKind::Plus,
            TokenKind::Id,
            TokenKind::Minus,
            TokenKind::Id,
            TokenKind::Divide,
            TokenKind::Id,
        ]
    );
}

// edge cases

#[test]
fn adjacent_operators() {
    // "!=" is NotEq, but "! =" is Not then Assign
    let toks = tokens("! =");
    assert_eq!(toks.len(), 2);
    assert_eq!(toks[0].kind(), TokenKind::Not);
    assert_eq!(toks[1].kind(), TokenKind::Assign);
}

#[test]
fn negative_number_is_minus_then_int() {
    // the lexer doesn't produce negative literals; "-5" is minus then int(5)
    let toks = tokens("-5");
    assert_eq!(toks.len(), 2);
    assert_eq!(toks[0].kind(), TokenKind::Minus);
    assert!(matches!(toks[1], Token::Int(5)));
}

#[test]
fn underscore_only_identifier() {
    // a lone underscore starts with non-uppercase, non-digit, non-operator
    // it's not lowercase either, so it should be invalid
    let toks = tokens("_");
    assert_eq!(toks[0].kind(), TokenKind::Invalid);
}

#[test]
fn echo_and_read_keywords() {
    let toks = tokens("echo read");
    assert_eq!(toks[0].kind(), TokenKind::Echo);
    assert_eq!(toks[1].kind(), TokenKind::Read);
}

#[test]
fn divide_not_confused_with_comment() {
    // "/" alone is Divide, "//" starts a comment
    let toks = tokens("a / b");
    assert_eq!(toks.len(), 3);
    assert_eq!(toks[1].kind(), TokenKind::Divide);
}

#[test]
fn multiline_program() {
    let program = "\
fn main() -> Int {
    let x = 10;
    let y = 20;
    return x + y;
}";
    let toks = tokens(program);
    // fn main ( ) -> Int { let x = 10 ; let y = 20 ; return x + y ; }
    assert_eq!(toks.len(), 23);
    assert_eq!(toks[0].kind(), TokenKind::Fn);
    assert_eq!(toks.last().unwrap().kind(), TokenKind::RBrace);
}

#[test]
fn spanned_token_kind_method() {
    let result = lex("42");
    assert_eq!(result[0].kind(), TokenKind::Int);
    assert_eq!(result[1].kind(), TokenKind::EoF);
}
