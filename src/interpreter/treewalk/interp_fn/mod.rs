use std::rc::Rc;

use crate::{
    frontend::parser::ast::{Expr, Stmt, Type},
    interpreter::treewalk::{Env, interp_expr, types::Value},
};

pub(super) fn interp_fn(name: &String, stmts: Vec<&Stmt>, env: Rc<Env>) -> Rc<Value> {
    let mut local_env = Rc::clone(&env);
    for stmt in stmts {
        match stmt {
            Stmt::Let(id, expr) => local_env = interp_let(id, expr, local_env),
            Stmt::Read(type_of_expr, id) => local_env = interp_read(type_of_expr, id, local_env),
            Stmt::Echo(_, expr) => interp_echo(expr, Rc::clone(&local_env)),
            Stmt::Return(expr) => return interp_expr::interp_expr(expr, Rc::clone(&local_env)),
        }
    }

    panic!("FATAL ERROR: function {} does not return", name)
}

fn interp_let(id: &String, expr: &Expr, env: Rc<Env>) -> Rc<Env> {
    let val = interp_expr::interp_expr(expr, Rc::clone(&env));
    Env::extend(env, id.clone(), val)
}

fn interp_read(type_of_expr: &Type, id: &String, env: Rc<Env>) -> Rc<Env> {
    todo!()
}

fn interp_echo(expr: &Expr, env: Rc<Env>) {
    let val = interp_expr::interp_expr(expr, env);
    println!("{val}");
}
