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

fn interp_let(id: &str, expr: &Expr, env: Rc<Env>) -> Rc<Env> {
    let val = interp_expr::interp_expr(expr, Rc::clone(&env));
    Env::extend(env, id.to_owned(), val)
}

fn interp_read(type_of_expr: &Type, id: &str, env: Rc<Env>) -> Rc<Env> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let val = match type_of_expr {
        Type::Int => Value::Int(input.trim().parse::<i64>().expect("Failed to parse integer")),
        Type::Float => Value::Float(input.trim().parse::<f64>().expect("Failed to parse float")),
        Type::Bool => Value::Bool(input.trim().parse::<bool>().expect("Failed to parse boolean")),
        _ => panic!("FATAL ERROR: cannot read value of type {type_of_expr}"),
    };

    Env::extend(env, id.to_owned(), val.into())
}

fn interp_echo(expr: &Expr, env: Rc<Env>) {
    let val = interp_expr::interp_expr(expr, env);
    println!("{val}");
}
