pub mod token;

#[cfg(test)]
mod tests;

use crate::frontend::lexer::token::{LexError, SpannedToken, Token, TokenKind};

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
const OPERATORS: [(&str, Token); 31] = [
    // three-char ops
    (">=.", Token::FGreaterThanOrEq),
    ("<=.", Token::FLessThanOrEq),
    // two-char ops
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

pub fn lex(program: &str) -> Vec<SpannedToken<'_>> {
    let mut lexer = Lexer::new(program);
    // todo - should this be in lexer struct, maybe?
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                tokens.push(token);
                if token.kind() == TokenKind::EoF {
                    break;
                }
            }
            Err(e) => {
                errors.push(e);
            }
        }
    }
    tokens
}

#[derive(Clone, Copy)]
struct Lexer<'a> {
    program: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    fn new(program: &'a str) -> Self {
        Self { program, offset: 0 }
    }

    fn next_token(&mut self) -> Result<SpannedToken<'a>, LexError<'a>> {
        let program = &self.program[self.offset..];
        self.eat_whitespace();

        let first_char = match program.chars().next() {
            Some(c) => c,
            None => return Ok(SpannedToken::new(Token::EoF, self.offset)),
        };

        // determine if the token is an operator
        for (op_str, op_token) in OPERATORS {
            if let Some(_) = program.strip_prefix(op_str) {
                let loc = self.offset;
                self.offset += op_str.len();
                return Ok(SpannedToken::new(op_token, loc));
            }
        }

        // determine if the token is a float/int
        if first_char.is_ascii_digit() {
            return self.read_num();
        }

        // determine if the token is a keyword or variable
        if first_char.is_lowercase() {
            return Ok(self.read_ident());
        }

        // determine if the token is a type identifier
        if first_char.is_uppercase() {
            return Ok(self.read_type_ident());
        }

        // determine if the token is a type parameter
        if first_char == '\'' {
            return Ok(self.read_type_param());
        }

        // otherwise, the token is invalid
        let loc = self.offset;
        self.offset += first_char.len_utf8();
        Ok(SpannedToken::new(Token::Invalid(first_char), loc))
    }

    /// Discards any whitespace or comments at the start of `program`.
    fn eat_whitespace(&mut self) {
        todo!()
        // let mut s = &self.program[self.offset..].chars().into_iter();
        // while let Some(c) = s.next()
        //     && c.is_whitespace()
        // {
        //     self.offset += c.len_utf8();
        // }
        // while s.starts_with("//") {
        //     s = match s.find('\n') {
        //         Some(i) => &s[i + 1..],
        //         None => &s[s.len()..],
        //     };
        //     s = s.trim_start();
        // }
    }

    fn read_num(&mut self) -> Result<SpannedToken<'a>, LexError<'a>> {
        // greedily grab all characters that form a number, allowing at most one '.'
        let mut seen_dot = false;
        let program = &self.program[self.offset..];
        let mut first_non_digit = program.len();
        for (index, char) in program.char_indices() {
            if char == '.' && !seen_dot {
                if program[index + 1..]
                    .chars()
                    .next()
                    .is_some_and(|next| next.is_ascii_digit())
                {
                    seen_dot = true;
                } else {
                    first_non_digit = index;
                    break;
                }
            } else if !char.is_ascii_digit() {
                first_non_digit = index;
                break;
            }
        }
        let digits = &program[..first_non_digit];
        let loc = self.offset;
        self.offset += first_non_digit;

        if seen_dot {
            match digits.parse::<f64>() {
                Ok(f) => Ok(SpannedToken::new(Token::Float(f), loc)),

                // this might not be reachable; i wonder if it'd be better not to check for "only
                // one dot" and just parse until we hit a non-digit, and let the parse fail if there
                // are multiple dots
                Err(_) => Err(LexError::InvalidFloat(digits)),
            }
        } else {
            match digits.parse::<i64>() {
                Ok(i) => Ok(SpannedToken::new(Token::Int(i), loc)),
                // i think this is just for overflow?
                Err(_) => Err(LexError::InvalidInt(digits)),
            }
        }
    }

    fn read_ident(&mut self) -> SpannedToken<'a> {
        let program = &self.program[self.offset..];

        // greedily grab all characters until we see something that's not a letter
        let mut first_non_letter = program.len();
        for (index, char) in program.char_indices() {
            if !(char.is_alphanumeric() || char == '_') {
                first_non_letter = index;
                break;
            }
        }

        let ident = &program[..first_non_letter];
        let loc = self.offset;
        self.offset += ident.len();

        // check against keywords, fallback to identifier (variable) if no match
        for (keyword_str, keyword_token) in KEYWORDS {
            if ident == keyword_str {
                return SpannedToken::new(keyword_token, loc);
            }
        }

        SpannedToken::new(Token::Id(ident), loc)
    }

    fn read_type_ident(&mut self) -> SpannedToken<'a> {
        // greedily grab all characters until we see something that's not a letter
        let program = &self.program[self.offset..];
        let mut first_non_letter = program.len();
        for (index, char) in program.char_indices() {
            if !char.is_alphabetic() {
                first_non_letter = index;
                break;
            }
        }
        let loc = self.offset;
        let ident = &program[..first_non_letter];
        self.offset += first_non_letter;

        // check against keywords, fallback to identifier (variable) if no match
        for (type_str, type_token) in TYPES {
            if ident == type_str {
                return SpannedToken::new(type_token, loc);
            }
        }

        // (Token::TypeId(ident), rest)
        SpannedToken::new(Token::TypeId(ident), loc)
    }

    fn read_type_param(&mut self) -> SpannedToken<'a> {
        // greedily grab the apostrophe-prefixed type parameter name
        let program = &self.program[self.offset..]; // skip leading apostrophe
        let mut first_non_lowercase = program.len();
        for (index, char) in program.char_indices() {
            if index == 0 {
                continue; // skip leading apostrophe
            }
            if !char.is_lowercase() {
                first_non_lowercase = index;
                break;
            }
        }

        let loc = self.offset;
        if first_non_lowercase > 1 {
            let param = &program[..first_non_lowercase];
            self.offset += first_non_lowercase;
            SpannedToken::new(Token::TypeParam(param), loc)
        } else {
            self.offset += 1;
            SpannedToken::new(Token::Invalid('\''), loc)
        }
    }
}
