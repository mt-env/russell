pub mod error;
pub mod types;

// 1 - lexes string into stream of tokens
pub mod lexer;

// 2 - parses stream of tokens into AST
pub mod parser;

// 3 - name resolution - desugars and generates an hir
pub mod resolution;

// 4 - type checking
pub mod typechecker;
