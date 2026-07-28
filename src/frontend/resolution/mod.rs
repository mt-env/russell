use crate::frontend::parser::ast::ParsedDefn;

pub mod resolve_defn;
pub mod resolve_expr;
pub mod resolve_stmt;
pub mod resolve_type;
pub mod types;

pub fn resolve(defns: Vec<ParsedDefn>) {
    todo!()
}
