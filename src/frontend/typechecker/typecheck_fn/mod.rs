use crate::frontend::{
    parser::ast::{ParsedExpr, ParsedStmt, Stmt, Type},
    typechecker::{
        infer,
        types::{Env, TypeResult, TypedStmt},
    },
};

pub fn typecheck_fn<'a>(name: &str, stmts: Vec<ParsedStmt<'a>>, env: &mut Env) -> TypedStmt<'a> {
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

fn typecheck_let<'a, 'b>(
    offset: usize,
    id: &'a str,
    expr: ParsedExpr<'a>,
    env: &mut Env,
) -> TypedStmt<'a> {
    let typed_expr = match infer::infer(expr, env) {
        Ok(expr) => expr,
        Err(_) => todo!(), // error recovery here
    };
    env.extend(id, typed_expr.ty());
    TypedStmt::make_let(offset, id, typed_expr)
}

fn typecheck_read<'a, 'b>(
    offset: usize,
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

fn typecheck_echo<'a>(offset: usize, id: &str, expr: ParsedExpr<'a>) -> TypedStmt<'a> {
    todo!()
}

fn typecheck_return<'a>(offset: usize, id: &str, expr: ParsedExpr<'a>) -> TypedStmt<'a> {
    todo!()
}
