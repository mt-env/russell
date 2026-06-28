use crate::frontend::{
    parser::ast::{ParsedExpr, ParsedStmt, Stmt, Type},
    typechecker::{
        check::unify,
        infer,
        types::{Env, TypedStmt},
    },
};

pub fn typecheck_fn<'a>(
    name: &str,
    stmts: Vec<ParsedStmt<'a>>,
    ret_ty: Type<'a>,
    env: &mut Env,
) -> TypedStmt<'a> {
    // check that each fn returns
    if !stmts.iter().any(|a| matches!(a.node, Stmt::Return(_))) {
        todo!() // error for failing to return
    }

    let mut typed_stmts = Vec::new();
    for stmt in stmts {
        let loc = stmt.offset;
        typed_stmts.push(match stmt.node {
            Stmt::Let(id, expr) => typecheck_let(loc, id, expr, env),
            Stmt::Read(typ, id) => typecheck_read(loc, typ, id, env),
            Stmt::Echo(typ, expr) => typecheck_echo(loc, typ, expr, env),
            Stmt::Return(expr) => typecheck_return(loc, ret_ty.clone(), expr, env),
        })
    }

    todo!()
}

fn typecheck_let<'a>(
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

fn typecheck_read<'a>(offset: usize, ty: Type<'a>, id: &'a str, env: &mut Env) -> TypedStmt<'a> {
    match ty {
        Type::Int | Type::Float | Type::Bool => env.extend(id, ty.clone().into()),
        _ => todo!(), // error handling - invalid read
    }

    TypedStmt::make_read(offset, ty, id)
}

fn typecheck_echo<'a>(
    offset: usize,
    typ: Type<'a>,
    expr: ParsedExpr<'a>,
    env: &mut Env,
) -> TypedStmt<'a> {
    match typ {
        Type::Int | Type::Bool | Type::Float => {
            let typed_expr = match infer::infer(expr, env) {
                Ok(expr) => expr,
                Err(_) => todo!(), // better error handling
            };
            // todo potentially unnecessary - see #28 on gh
            unify(typ.into(), typed_expr.ty()).unwrap();
            TypedStmt::make_echo(offset, typed_expr)
        }
        _ => todo!(), // error handling - potentially invalid echo? see #27 on gh
    }
}

fn typecheck_return<'a>(
    offset: usize,
    expected_type: Type<'a>,
    expr: ParsedExpr<'a>,
    env: &mut Env,
) -> TypedStmt<'a> {
    let typed_expr = match infer::infer(expr, env) {
        Ok(expr) => expr,
        Err(_) => todo!(), // better error handling
    };
    unify(expected_type.into(), typed_expr.ty()).unwrap(); // todo better error handling
    TypedStmt::make_return(offset, typed_expr)
}
