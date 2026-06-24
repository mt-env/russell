use crate::frontend::{
    parser::ast::{ParsedExpr, ParsedStmt, Stmt, Type},
    typechecker::{
        infer,
        types::{Env, TypeResult, TypedStmt},
    },
};

pub fn typecheck_fn<'a>(name: &str, stmts: Vec<ParsedStmt<'a>>, env: &mut Env) -> TypedStmt<'a> {
    // check that each fn returns
    if stmts
        .iter()
        .map(|a| matches!(a.node, Stmt::Return(_)))
        .count()
        <= 0
    {
        todo!() // error for failing to return
    }

    let mut typed_stmts = Vec::new();
    for stmt in stmts {
        let loc = stmt.offset;
        typed_stmts.push(match stmt.node {
            Stmt::Let(id, expr) => typecheck_let(loc, id, expr, env),
            Stmt::Read(typ, id) => typecheck_read(loc, typ, id, env),
            Stmt::Echo(typ, expr) => typecheck_echo(loc, typ, expr, env),
            Stmt::Return(expr) => todo!(),
        })
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
    ty: Type<'a>,
    id: &'a str,
    env: &'b mut Env,
) -> TypedStmt<'a> {
    match ty {
        Type::Int | Type::Float | Type::Bool => env.extend(id, ty.clone().into()),
        _ => todo!(), // error handling - invalid read
    }

    TypedStmt::make_read(offset, ty, id)
}

fn typecheck_echo<'a, 'b>(
    offset: usize,
    typ: Type<'a>,
    expr: ParsedExpr<'a>,
    env: &'b mut Env,
) -> TypedStmt<'a> {
    match ty {}
}

fn typecheck_return<'a>(offset: usize, id: &str, expr: ParsedExpr<'a>) -> TypedStmt<'a> {
    todo!()
}
