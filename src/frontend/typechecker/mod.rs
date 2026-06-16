use crate::frontend::{parser::ast::ParsedDefn, typechecker::types::TypedDefn};

pub mod typecheck_expr;
pub mod typecheck_fn;
pub mod types;

pub fn typecheck(_defns: Vec<ParsedDefn>) -> Vec<TypedDefn> {
    todo!()
}
