use std::{collections::HashMap, rc::Rc};

use crate::frontend::parser::ast::{Defn, ExprKind, ParsedDefn, ParsedExpr, Type};
use crate::interpreter::treewalk::types::{Env, Value};

mod interp_expr;
mod interp_fn;
mod types;

pub fn interp(defns: Vec<ParsedDefn>) {
    let global_env = process_global_env(defns);
    let main_call = ParsedExpr::new(
        // todo - is this the right offset to use for the main call?
        0,
        ExprKind::Call(Box::new(ParsedExpr::new(0, ExprKind::Id("main"))), Vec::new()),
    );
    interp_expr::interp_expr(&main_call, global_env);
}

fn process_global_env(defns: Vec<ParsedDefn>) -> Rc<Env> {
    let mut map = HashMap::new();
    for defn in defns {
        match defn.node {
            Defn::Typedef { id, ty_vars: _, arms } => {
                for (name, bindings) in arms {
                    map.insert(name, Rc::new(Value::Constructor(name, Type::TypeId(id), bindings)));
                }
            }
            Defn::Fn(id, bindings, _, stmts) => {
                map.insert(id, Rc::new(Value::Fn(id, bindings, stmts)));
            }
        }
    }
    Rc::new(Env::Global(map))
}
