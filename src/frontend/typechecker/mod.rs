use crate::frontend::{parser::ast::ParsedDefn, typechecker::types::TypedDefn};

pub mod types;
pub mod typecheck_expr;
pub mod typecheck_fn;


fn typecheck(defns: Vec<ParsedDefn>) -> Vec<TypedDefn> {
    todo!()
}
