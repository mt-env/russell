use crate::frontend::{
    parser::ast::ParsedExpr,
    typechecker::{
        inference::context::Context,
        types::{TypeResult, TypeValue, TypedExpr},
    },
};

#[cfg(test)]
mod tests;

pub(super) fn check<'a>(
    expr: ParsedExpr<'a>,
    expected: TypeValue,
    ctx: &Context,
) -> TypeResult<TypedExpr<'a>> {
    todo!()
}
