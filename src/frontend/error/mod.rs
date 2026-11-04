pub mod lex_error;
pub mod parse_error;

use std::fmt;

pub trait CompilerError<'src>: fmt::Display {
    fn offset(&self) -> usize;
}

pub fn report_all<'src>(
    errors: &[impl CompilerError<'src>],
    source: &'src str,
    filename: &'src str,
) -> String {
    errors
        .iter()
        .map(|error| report(error, source, filename))
        .collect::<Vec<_>>()
        .join("\n\n")
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
pub fn report<'src>(
    error: &impl CompilerError<'src>,
    source: &'src str,
    filename: &'src str,
) -> String {
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
    let end = source[offset..]
        .find('\n')
        .map(|i| offset + i)
        .unwrap_or(source.len());
    &source[start..end]
}
