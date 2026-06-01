use crate::frontend::{
    parser::ast::ParsedExpr,
    typechecker::types::{Env, TypeResult, TypedExpr},
};

pub(super) fn typecheck_expr(_expr: ParsedExpr) -> TypedExpr {
    todo!()
}
