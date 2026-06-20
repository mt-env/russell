use crate::frontend::{
    parser::ast::ParsedExpr,
    typechecker::types::{Env, TypeResult, TypeValue, TypedExpr},
};

#[cfg(test)]
mod tests;

pub(super) fn check<'a>(
    expr: ParsedExpr<'a>,
    expected: TypeValue,
    env: &Env,
) -> TypeResult<TypedExpr<'a>> {
    todo!()
}

pub(super) fn unify(expected: TypeValue, actual: TypeValue) -> TypeResult<()> {
    todo!()
}
