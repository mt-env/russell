use crate::frontend::{parser::ast::ParsedDefn, typechecker::types::TypedDefn};

pub mod typecheck_expr;
pub mod typecheck_fn;
pub mod types;

fn typecheck(defns: Vec<ParsedDefn>) -> Vec<TypedDefn> {
    todo!()
}
