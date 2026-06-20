use crate::frontend::{
    parser::ast::ParsedExpr,
    typechecker::types::{Env, TypeResult, TypeValue, TypedExpr},
};

#[cfg(test)]
mod tests;

pub(super) fn check(expr: TypedExpr, expected: TypeValue, env: &Env) -> TypeResult<TypeValue> {
    todo!()
}

pub(super) fn unify(expected: TypeValue, actual: TypeValue) -> TypeResult<()> {
    todo!()
}
