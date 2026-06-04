pub mod token;

#[cfg(test)]
mod tests;

use crate::frontend::lexer::token::{SpannedToken, Token};

// reserved keywords
const KEYWORDS: [(&str, Token); 12] = [
    ("echo", Token::Echo),
    ("else", Token::Else),
    ("false", Token::Bool(false)),
    ("fn", Token::Fn),
    ("if", Token::If),
    ("let", Token::Let),
    ("match", Token::Match),
    ("read", Token::Read),
    ("return", Token::Return),
    ("then", Token::Then),
    ("true", Token::Bool(true)),
    ("typedef", Token::Typedef),
];

// type keywords
const TYPES: [(&str, Token); 3] = [
    ("Int", Token::IntType),
    ("Float", Token::FloatType),
    ("Bool", Token::BoolType),
];

// operators
const OPERATORS: [(&str, Token); 23] = [
    // two-char ops
    ("!=", Token::NotEq),
    ("&&", Token::And),
    ("->", Token::Arrow),
    ("<=", Token::LessThanOrEq),
    ("==", Token::Eq),
    (">=", Token::GreaterThanOrEq),
    ("|>", Token::Pipe),
    ("||", Token::Or),
    // one-char ops
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

/// Given the entire program as a string, lexes it into a vector of spanned tokens.
pub fn lex(program: &str) -> Vec<SpannedToken<'_>> {
    let base = program.as_ptr() as usize;
    let mut tokens = Vec::new();
    let mut rest = program;

    loop {
        rest = eat_whitespace(rest);
        let offset = rest.as_ptr() as usize - base;
        let (token, remaining) = next_token(rest);
        let done = matches!(token, Token::EoF);
        tokens.push(SpannedToken { token, offset });
        rest = remaining;
        if done {
            break;
        }
    }

    tokens
}

/// Lexes the next token in the given program.
/// Returns the token, and the rest of the program, which has not been lexed.
/// Assumes leading whitespace has already been consumed.
fn next_token(program: &str) -> (Token<'_>, &str) {
    let first_char = match program.chars().next() {
        Some(c) => c,
        None => return (Token::EoF, program),
    };

    // determine if the token is an operator
    for (op_str, op_token) in OPERATORS {
        if let Some(rest) = program.strip_prefix(op_str) {
            return (op_token, rest);
        }
    }

    // determine if the token is a float/int
    if first_char.is_ascii_digit() {
        return read_num(program);
    }

    // determine if the token is a keyword or variable
    if first_char.is_lowercase() {
        return read_ident(program);
    }

    // determine if the token is a type identifier
    if first_char.is_uppercase() {
        return read_type_ident(program);
    }

    // otherwise, the token is invalid
    (Token::Invalid(first_char), &program[first_char.len_utf8()..])
}

/// Discards any whitespace or comments at the start of `program`.
fn eat_whitespace(program: &str) -> &str {
    let mut s = program.trim_start();
    while s.starts_with("//") {
        s = match s.find('\n') {
            Some(i) => &s[i + 1..],
            None => &s[s.len()..],
        };
        s = s.trim_start();
    }
    s
}

fn read_num(program: &str) -> (Token<'_>, &str) {
    // greedily grab all characters that form a number, allowing at most one '.'
    let mut seen_dot = false;
    let mut first_non_digit = program.len();
    for (index, char) in program.char_indices() {
        if char == '.' && !seen_dot {
            seen_dot = true;
        } else if !char.is_ascii_digit() {
            first_non_digit = index;
            break;
        }
    }
    let digits = &program[..first_non_digit];
    let rest = &program[first_non_digit..];

    if seen_dot {
        (Token::Float(digits.parse::<f64>().unwrap()), rest)
    } else {
        match digits.parse::<i64>() {
            Ok(num) => (Token::Int(num), rest),
            Err(_) => (Token::Overflow(digits), rest),
        }
    }
}

fn read_ident(program: &str) -> (Token<'_>, &str) {
    // greedily grab all characters until we see something that's not a letter
    let mut first_non_letter = program.len();
    for (index, char) in program.char_indices() {
        if !(char.is_alphanumeric() || char == '_') {
            first_non_letter = index;
            break;
        }
    }
    let ident = &program[..first_non_letter];
    let rest = &program[first_non_letter..];

    // check against keywords, fallback to identifier (variable) if no match
    for (keyword_str, keyword_token) in KEYWORDS {
        if ident == keyword_str {
            return (keyword_token, rest);
        }
    }

    (Token::Id(ident), rest)
}

fn read_type_ident(program: &str) -> (Token<'_>, &str) {
    // greedily grab all characters until we see something that's not a letter
    let mut first_non_letter = program.len();
    for (index, char) in program.char_indices() {
        if !char.is_alphabetic() {
            first_non_letter = index;
            break;
        }
    }
    let ident = &program[..first_non_letter];
    let rest = &program[first_non_letter..];

    // check against keywords, fallback to identifier (variable) if no match
    for (type_str, type_token) in TYPES {
        if ident == type_str {
            return (type_token, rest);
        }
    }

    (Token::TypeId(ident), rest)
}
