use crate::frontend::{
    parser::ast::ParsedDefn,
    typechecker::types::{Env, TypedDefn},
};

use super::parser::ast::Defn;

pub mod check;
pub mod context;
pub mod infer;
pub mod typecheck_fn;
pub mod types;

pub fn typecheck(_defns: Vec<ParsedDefn>) -> Vec<TypedDefn> {
    todo!()
}

fn process_global_env(defns: Vec<ParsedDefn>) -> Env {
    for defn in defns {
        match defn.node {
            Defn::Typedef(_, _) => todo!(),
            Defn::Fn(_, _, _, _) => todo!(),
        }
    }
    todo!()
}
