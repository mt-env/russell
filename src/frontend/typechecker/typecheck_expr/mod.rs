use crate::frontend::{parser::ast::ParsedExpr, typechecker::types::{Env, TypeResult, TypedExpr}};


pub(super) fn typeck_expr(expr: ParsedExpr, env: Env) -> TypeResult<TypedExpr> {
    todo!()
}
