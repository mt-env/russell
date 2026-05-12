use std::rc::Rc;

use crate::frontend::parser::ast::{Binding, Defn, Expr, ExprKind, ParsedDefn, ParsedStmt, Type};
use crate::interpreter::treewalk::types::{Env, Value};

mod interp_expr;
mod interp_fn;
mod types;

pub fn interp(defns: Vec<ParsedDefn>) {
    let global_env = process_global_env(defns);
    let main_call = Expr::parsed(ExprKind::Call(
        Box::new(Expr::parsed(ExprKind::Id("main".to_string()))),
        Vec::new(),
    ));
    interp_expr::interp_expr(&main_call, global_env);
}

fn process_global_env(defns: Vec<ParsedDefn>) -> Rc<Env> {
    let mut env = Env::new().into();
    for defn in defns {
        env = match defn {
            Defn::Typedef(adt_type, arms) => add_typedef(env, Type::TypeId(adt_type), arms),
            Defn::Fn(id, bindings, _, stmts) => add_fn_def(env, id, bindings, stmts),
        }
    }

    env
}

fn add_typedef(env: Rc<Env>, adt_type: Type, bindings: Vec<(String, Vec<Binding>)>) -> Rc<Env> {
    let mut new_env = env;
    for (name, bindings) in bindings {
        new_env = Env::extend(
            new_env,
            name.clone(),
            Value::Constructor(name, adt_type.clone(), bindings).into()
        )
    }

    new_env
}

fn add_fn_def(env: Rc<Env>, id: String, bindings: Vec<Binding>, stmts: Vec<ParsedStmt>) -> Rc<Env> {
    Env::extend(env, id.clone(), Value::Fn(id, bindings, stmts).into())
}
