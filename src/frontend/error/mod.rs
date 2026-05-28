pub mod lex_error;
pub mod parse_error;

use std::fmt;

use lex_error::LexError;
use parse_error::ParseError;

#[derive(Debug)]
pub enum CompilerError<'a> {
    Lex(LexError),
    Parse(ParseError<'a>),
}

impl CompilerError<'_> {
    fn offset(&self) -> usize {
        match self {
            CompilerError::Lex(e) => e.offset,
            CompilerError::Parse(e) => e.offset,
        }
    }
}

impl fmt::Display for CompilerError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Lex(e) => write!(f, "{}", e),
            CompilerError::Parse(e) => write!(f, "{}", e),
        }
    }
}

impl From<LexError> for CompilerError<'_> {
    fn from(e: LexError) -> Self {
        CompilerError::Lex(e)
    }
}

impl<'a> From<ParseError<'a>> for CompilerError<'a> {
    fn from(e: ParseError<'a>) -> Self {
        CompilerError::Parse(e)
    }
}

/// Format a compiler error with source context for display.
///
/// Produces output like:
/// ```text
/// error: invalid character '@'
///  --> test.rsl:3:5
///   |
/// 3 | let @x = 5;
///   |     ^
/// ```
pub fn report(error: &CompilerError, source: &str, filename: &str) -> String {
    let (line, col) = offset_to_location(source, error.offset());
    let src_line = source_line_at(source, error.offset());
    let gutter = line.to_string().len();

    format!(
        "error: {}\n{}--> {}:{}:{}\n{} |\n{} | {}\n{} | {}^",
        error,
        " ".repeat(gutter + 1),
        filename,
        line,
        col,
        " ".repeat(gutter),
        line,
        src_line,
        " ".repeat(gutter),
        " ".repeat(col - 1),
    )
}

/// Convert a byte offset to a 1-indexed (line, column) pair.
pub fn offset_to_location(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

/// Return the source line containing the given byte offset.
pub fn source_line_at(source: &str, offset: usize) -> &str {
    let start = source[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let end = source[offset..].find('\n').map(|i| offset + i).unwrap_or(source.len());
    &source[start..end]
}
