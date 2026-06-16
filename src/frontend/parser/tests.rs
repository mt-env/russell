use crate::frontend::lexer::lex;
use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::*;

fn parser_from(input: &str) -> super::Parser {
    super::Parser::new(lex(input))
}

// ─── parse() ────────────────────────────────────────────────────────

#[test]
fn parse_empty_program() {
    let defns = super::parse(lex(""));
    assert!(defns.is_empty());
}

#[test]
fn parse_single_fn() {
    let defns = super::parse(lex("fn main() -> Int { return 0; }"));
    assert_eq!(defns.len(), 1);
    assert_eq!(
        defns[0],
        ParsedDefn::make_fn(
            0,
            "main".into(),
            vec![],
            Type::Int,
            vec![ParsedStmt::make_return(19, ParsedExpr::new(26, ExprKind::Int(0)))]
        )
    );
}

#[test]
fn parse_single_typedef() {
    let defns = super::parse(lex("typedef Unit { unit() }"));
    assert_eq!(defns.len(), 1);
    assert_eq!(
        defns[0],
        ParsedDefn::make_typedef(0, "Unit".into(), vec![("unit".into(), vec![])])
    );
}

#[test]
fn parse_multiple_definitions() {
    let src = "typedef Color { red(), blue() } fn main() -> Int { return 0; }";
    let defns = super::parse(lex(src));
    assert_eq!(defns.len(), 2);
    assert!(matches!(&defns[0].node, Defn::Typedef(name, ..) if *name == "Color"));
    assert!(matches!(&defns[1].node, Defn::Fn(name, ..) if *name == "main"));
}

#[test]
fn parse_multiple_fns() {
    let src = "\
        fn foo() -> Int { return 1; } \
        fn bar() -> Int { return 2; } \
        fn baz() -> Int { return 3; }";
    let defns = super::parse(lex(src));
    assert_eq!(defns.len(), 3);
    assert!(matches!(&defns[0].node, Defn::Fn(name, ..) if *name == "foo"));
    assert!(matches!(&defns[1].node, Defn::Fn(name, ..) if *name == "bar"));
    assert!(matches!(&defns[2].node, Defn::Fn(name, ..) if *name == "baz"));
}

#[test]
fn parse_typedef_then_fn_using_it() {
    let src = "\
        typedef Option { some(x: Int), none() } \
        fn unwrap(opt: Option) -> Int { \
            return match opt { some(x: Int) -> x, none() -> 0 }; \
        }";
    let defns = super::parse(lex(src));
    assert_eq!(defns.len(), 2);
    assert!(matches!(&defns[0].node, Defn::Typedef(name, ctors) if *name == "Option" && ctors.len() == 2));
    assert!(matches!(&defns[1].node, Defn::Fn(name, ..) if *name == "unwrap"));
}

// ─── peek ───────────────────────────────────────────────────────────

#[test]
fn peek_does_not_consume() {
    let mut p = parser_from("42");
    assert_eq!(p.peek().kind(), TokenKind::Int);
    assert_eq!(p.peek().kind(), TokenKind::Int);
}

#[test]
fn peek_returns_eof_on_empty() {
    let mut p = parser_from("");
    assert_eq!(p.peek().kind(), TokenKind::EoF);
}

#[test]
fn peek_shows_first_token() {
    let mut p = parser_from("fn foo");
    assert_eq!(p.peek().kind(), TokenKind::Fn);
}

// ─── advance ────────────────────────────────────────────────────────

#[test]
fn advance_consumes_token() {
    let mut p = parser_from("42");
    let t = p.advance();
    assert_eq!(t.kind(), TokenKind::Int);
    assert_eq!(p.peek().kind(), TokenKind::EoF);
}

#[test]
fn advance_sequence() {
    let mut p = parser_from("1 + 2");
    assert_eq!(p.advance().kind(), TokenKind::Int);
    assert_eq!(p.advance().kind(), TokenKind::Plus);
    assert_eq!(p.advance().kind(), TokenKind::Int);
    assert_eq!(p.peek().kind(), TokenKind::EoF);
}

// ─── expect ─────────────────────────────────────────────────────────

#[test]
fn expect_success() {
    let mut p = parser_from("(");
    assert!(p.expect(TokenKind::LParen).is_ok());
    assert_eq!(p.peek().kind(), TokenKind::EoF);
}

#[test]
fn expect_failure_does_not_consume() {
    let mut p = parser_from("(");
    assert!(p.expect(TokenKind::RParen).is_err());
    assert_eq!(p.peek().kind(), TokenKind::LParen);
}

#[test]
fn expect_sequence() {
    let mut p = parser_from("( )");
    assert!(p.expect(TokenKind::LParen).is_ok());
    assert!(p.expect(TokenKind::RParen).is_ok());
    assert_eq!(p.peek().kind(), TokenKind::EoF);
}

// ─── expect_id ──────────────────────────────────────────────────────

#[test]
fn expect_id_success() {
    let mut p = parser_from("foo");
    assert_eq!(p.expect_id().unwrap(), "foo");
}

#[test]
fn expect_id_failure() {
    let mut p = parser_from("42");
    assert!(p.expect_id().is_err());
}

#[test]
fn expect_id_extracts_name() {
    let mut p = parser_from("my_var");
    assert_eq!(p.expect_id().unwrap(), "my_var");
}

// ─── expect_int ─────────────────────────────────────────────────────

#[test]
fn expect_int_success() {
    let mut p = parser_from("42");
    assert_eq!(p.expect_int().unwrap(), 42);
}

#[test]
fn expect_int_zero() {
    let mut p = parser_from("0");
    assert_eq!(p.expect_int().unwrap(), 0);
}

#[test]
fn expect_int_failure_on_id() {
    let mut p = parser_from("foo");
    assert!(p.expect_int().is_err());
}

// ─── expect_float ───────────────────────────────────────────────────

#[test]
fn expect_float_success() {
    let mut p = parser_from("3.14");
    let val = p.expect_float().unwrap();
    assert!((val - 3.14).abs() < 1e-10);
}

#[test]
fn expect_float_failure_on_int() {
    let mut p = parser_from("42");
    assert!(p.expect_float().is_err());
}

// ─── expect_bool ────────────────────────────────────────────────────

#[test]
fn expect_bool_true() {
    let mut p = parser_from("true");
    assert_eq!(p.expect_bool().unwrap(), true);
}

#[test]
fn expect_bool_false() {
    let mut p = parser_from("false");
    assert_eq!(p.expect_bool().unwrap(), false);
}

#[test]
fn expect_bool_failure() {
    let mut p = parser_from("42");
    assert!(p.expect_bool().is_err());
}

// ─── expect_typeid ──────────────────────────────────────────────────

#[test]
fn expect_typeid_success() {
    let mut p = parser_from("MyType");
    assert_eq!(p.expect_typeid().unwrap(), "MyType");
}

#[test]
fn expect_typeid_builtin() {
    // Int, Float, Bool are type keywords, not TypeId
    let mut p = parser_from("Int");
    assert!(p.expect_typeid().is_err());
}

#[test]
fn expect_typeid_failure_on_id() {
    let mut p = parser_from("foo");
    assert!(p.expect_typeid().is_err());
}

// ─── expect_many ────────────────────────────────────────────────────

#[test]
fn expect_many_first_match() {
    let mut p = parser_from("+");
    let tok = p.expect_many(&[TokenKind::Plus, TokenKind::Minus]).unwrap();
    assert_eq!(tok.kind(), TokenKind::Plus);
}

#[test]
fn expect_many_second_match() {
    let mut p = parser_from("-");
    let tok = p.expect_many(&[TokenKind::Plus, TokenKind::Minus]).unwrap();
    assert_eq!(tok.kind(), TokenKind::Minus);
}

#[test]
fn expect_many_no_match() {
    let mut p = parser_from("*");
    assert!(p.expect_many(&[TokenKind::Plus, TokenKind::Minus]).is_err());
}

#[test]
fn expect_many_consumes_on_success() {
    let mut p = parser_from("+ -");
    p.expect_many(&[TokenKind::Plus]).unwrap();
    assert_eq!(p.peek().kind(), TokenKind::Minus);
}

// ─── take_if ────────────────────────────────────────────────────────

#[test]
fn take_if_match_consumes() {
    let mut p = parser_from("+");
    let tok = p.take_if(TokenKind::Plus);
    assert!(matches!(tok, Some(Token::Plus)));
    assert_eq!(p.peek().kind(), TokenKind::EoF);
}

#[test]
fn take_if_no_match_does_not_consume() {
    let mut p = parser_from("+");
    let tok = p.take_if(TokenKind::Minus);
    assert!(tok.is_none());
    assert_eq!(p.peek().kind(), TokenKind::Plus);
}

#[test]
fn take_if_extracts_value() {
    let mut p = parser_from("foo");
    let tok = p.take_if(TokenKind::Id);
    match tok {
        Some(Token::Id(name)) => assert_eq!(name, "foo"),
        other => panic!("expected Some(Id), got {:?}", other),
    }
}
