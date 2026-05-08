use std::rc::Rc;

use crate::frontend::parser::ast::{Binding, Defn, Expr, Stmt, Type};
use crate::interpreter::treewalk::types::{Env, Value};

mod interp_expr;
mod interp_fn;
mod types;

pub fn interp(defns: Vec<Defn>) {
    let global_env = process_global_env(defns);
    interp_expr::interp_expr(
        &Expr::Call(Box::new(Expr::Id("main".to_string())), Vec::new()),
        global_env,
    );
}

/// process the global environment by adding all type definitions and function definitions to it.
fn process_global_env(defns: Vec<Defn>) -> Rc<Env> {
    let mut env = Env::new().into();
    for defn in defns {
        env = match defn {
            Defn::Typedef(adt_type, arms) => add_typedef(env, Type::TypeId(adt_type), arms),
            Defn::Fn(id, bindings, _, stmts) => add_fn_def(env, id, bindings, stmts),
        }
    }

    env
}

/// add a type definition to the environment. this involves adding the type constructor and all of
/// its arms to the environment.
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

/// add a function definition to the environment. this involves adding the function name and its
/// implementation to the environment.
fn add_fn_def(env: Rc<Env>, id: String, bindings: Vec<Binding>, stmts: Vec<Stmt>) -> Rc<Env> {
    Env::extend(env, id.clone(), Value::Fn(id, bindings, stmts).into())
}
