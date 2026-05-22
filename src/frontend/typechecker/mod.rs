use crate::frontend::{parser::ast::ParsedDefn, typechecker::types::{Env, TypedDefn}};

use super::parser::ast::Defn;

pub mod types;
pub mod typecheck_expr;
pub mod typecheck_fn;


pub fn typecheck(defns: Vec<ParsedDefn>) -> Vec<TypedDefn> {
    todo!()
}

fn process_global_env(defns: Vec<ParsedDefn>) -> Env {
    for defn in defns {
        match defn {
            Defn::Typedef(name, vars) => todo!(),
            Defn::Fn(name, bindings, ty, stmts) => todo!()
        }
    }
    todo!()
}
