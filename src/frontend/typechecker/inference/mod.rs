use crate::frontend::{
    parser::ast::{Defn, ParsedDefn},
    typechecker::types::{Env, TypedDefn},
};

pub mod check;
pub mod context;
pub mod infer;
pub mod typecheck_fn;

pub fn typecheck(_defns: Vec<ParsedDefn>) -> Vec<TypedDefn> {
    todo!()
}

fn process_global_env(defns: Vec<ParsedDefn>) -> Env {
    for defn in defns {
        match defn.node {
            Defn::Typedef {
                id: _,
                ty_vars: _,
                arms: _,
            } => todo!(),
            Defn::Fn {
                name: _,
                bindings: _,
                ret_ty: _,
                body: _,
            } => todo!(),
        }
    }
    todo!()
}
