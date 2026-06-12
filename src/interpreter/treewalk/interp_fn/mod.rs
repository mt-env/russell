use std::rc::Rc;

use crate::{
    frontend::parser::ast::{ParsedExpr, ParsedStmt, StmtKind, Type},
    interpreter::treewalk::{Env, interp_expr, types::Value},
};

pub(super) fn interp_fn<'a>(name: &str, stmts: &[ParsedStmt<'a>], env: Rc<Env<'a>>) -> Rc<Value<'a>> {
    let mut local_env = Rc::clone(&env);
    for stmt in stmts {
        match &stmt.kind {
            StmtKind::Let(id, expr) => local_env = interp_let(id, expr, local_env),
            StmtKind::Read(type_of_expr, id) => local_env = interp_read(type_of_expr, id, local_env),
            StmtKind::Echo(_, expr) => interp_echo(expr, Rc::clone(&local_env)),
            StmtKind::Return(expr) => return interp_expr::interp_expr(expr, Rc::clone(&local_env)),
        }
    }

    panic!("FATAL ERROR: function {} does not return", name)
}

fn interp_let<'a>(id: &'a str, expr: &ParsedExpr<'a>, env: Rc<Env<'a>>) -> Rc<Env<'a>> {
    let val = interp_expr::interp_expr(expr, Rc::clone(&env));
    Env::extend(env, id, val)
}

fn interp_read<'a>(type_of_expr: &Type<'a>, id: &'a str, env: Rc<Env<'a>>) -> Rc<Env<'a>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let val = match type_of_expr {
        Type::Int => Value::Int(input.trim().parse::<i64>().expect("Failed to parse integer")),
        Type::Float => Value::Float(input.trim().parse::<f64>().expect("Failed to parse float")),
        Type::Bool => Value::Bool(input.trim().parse::<bool>().expect("Failed to parse boolean")),
        _ => panic!("FATAL ERROR: cannot read value of type {type_of_expr}"),
    };

    Env::extend(env, id, val.into())
}

fn interp_echo<'a>(expr: &ParsedExpr<'a>, env: Rc<Env<'a>>) {
    let val = interp_expr::interp_expr(expr, env);
    println!("{val}");
}
