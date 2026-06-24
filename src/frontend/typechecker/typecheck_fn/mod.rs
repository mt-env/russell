use crate::frontend::{
    parser::ast::{ParsedExpr, ParsedStmt, Stmt, Type},
    typechecker::types::{Env, TypedStmt},
};

pub fn typecheck_fn<'a>(name: &str, stmts: &[ParsedStmt<'a>], env: &Env) -> TypedStmt<'a> {
    let mut local_env = env;
    for stmt in stmts {
        match &stmt.node {
            Stmt::Let(id, expr) => {}
            Stmt::Read(type_of_expr, id) => todo!(),
            Stmt::Echo(_, expr) => todo!(),
            Stmt::Return(expr) => todo!(),
        }
    }

    todo!()
}

fn typecheck_let<'a>(id: &str, expr: ParsedExpr<'a>) -> TypedStmt<'a> {
    todo!()
}

fn typecheck_read<'a, 'b>(
    ty: &Type,
    expr: ParsedExpr<'a>,
    env: &'b Env,
) -> (TypedStmt<'a>, &'b Env) {
    match ty {
        Type::Int => todo!(),
        Type::Float => todo!(),
        Type::Bool => todo!(),
        Type::TypeId(_) => todo!(),
        Type::Fn(_, _) => todo!(),
    }
}

fn typecheck_echo<'a>(id: &str, expr: ParsedExpr<'a>) -> TypedStmt<'a> {
    todo!()
}

fn typecheck_return<'a>(id: &str, expr: ParsedExpr<'a>) -> TypedStmt<'a> {
    todo!()
}
