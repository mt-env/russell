pub mod token;

#[cfg(test)]
mod tests;

use crate::frontend::lexer::token::{LexError, SpannedLexError, SpannedToken, Token, TokenKind};

const KEYWORDS: [(&str, Token); 15] = [
    // reserved keywords
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
    // type keywords
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

pub fn lex(program: &str) -> Result<Vec<SpannedToken<'_>>, Vec<SpannedLexError<'_>>> {
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

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

struct Lexer<'a> {
    program: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    fn new(program: &'a str) -> Self {
        Self { program, offset: 0 }
    }

    fn next_token(&mut self) -> Result<SpannedToken<'a>, SpannedLexError<'a>> {
        self.eat_whitespace();

        let program = &self.program[self.offset..];
        let first_char = match program.chars().next() {
            Some(c) => c,
            None => return Ok(SpannedToken::new(Token::EoF, self.offset)),
        };

        // determine if the token is an operator
        for (op_str, op_token) in OPERATORS {
            if program.starts_with(op_str) {
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
            return Ok(self.read_ident(Token::Id));
        }

        // determine if the token is a type identifier
        if first_char.is_uppercase() {
            return Ok(self.read_ident(Token::TypeId));
        }

        // determine if the token is a type parameter
        if first_char == '\'' {
            return self.read_type_param();
        }

        // otherwise, the token is invalid
        let loc = self.offset;
        self.offset += first_char.len_utf8();
        Err(SpannedLexError::new(LexError::InvalidChar(first_char), loc))
    }

    fn eat_whitespace(&mut self) {
        let remainder = &self.program[self.offset..];
        self.offset += remainder.len() - remainder.trim_start().len();

        while self.program[self.offset..].starts_with("//") {
            let remainder = &self.program[self.offset..];
            self.offset += remainder
                .find('\n')
                .map_or(remainder.len(), |index| index + 1);

            let remainder = &self.program[self.offset..];
            self.offset += remainder.len() - remainder.trim_start().len();
        }
    }

    fn read_num(&mut self) -> Result<SpannedToken<'a>, SpannedLexError<'a>> {
        // greedily grab all characters that form a number (numbers and dots)
        let mut seen_dot = false;
        let program = &self.program[self.offset..];
        let mut first_non_digit = program.len();
        for (index, char) in program.char_indices() {
            if char == '.' {
                seen_dot = true;
            } else if !(char.is_numeric()) {
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
                Err(_) => Err(SpannedLexError::new(LexError::InvalidFloat(digits), loc)),
            }
        } else {
            match digits.parse::<i64>() {
                Ok(i) => Ok(SpannedToken::new(Token::Int(i), loc)),
                Err(_) => Err(SpannedLexError::new(LexError::InvalidInt(digits), loc)),
            }
        }
    }

    fn read_ident(&mut self, make: impl Fn(&'a str) -> Token) -> SpannedToken<'a> {
        let program = &self.program[self.offset..];

        // greedily grab all characters until we see something that's not a letter
        let is_end = |char: char| !(char.is_alphanumeric() || char == '_');
        let end = program.find(is_end).unwrap_or(program.len());

        let ident = &program[..end];
        let loc = self.offset;
        self.offset += ident.len();

        // check against keywords, fallback to identifier (variable) if no match
        for (keyword_str, keyword_token) in KEYWORDS {
            if ident == keyword_str {
                return SpannedToken::new(keyword_token, loc);
            }
        }

        SpannedToken::new(make(ident), loc)
    }

    fn read_type_param(&mut self) -> Result<SpannedToken<'a>, SpannedLexError<'a>> {
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
            Ok(SpannedToken::new(Token::TypeParam(param), loc))
        } else {
            self.offset += 1;
            Err(SpannedLexError::new(LexError::InvalidChar('\''), loc))
        }
    }
}
