use crate::frontend::{
    parser::ast::ParsedExpr,
    typechecker::{
        inference::context::{Context, InferredExpr, TypeId},
        types::{TypeResult, TypeValue, TypedExpr},
    },
};

#[cfg(test)]
mod tests;

pub(super) fn check<'a>(
    expr: ParsedExpr<'a>,
    expected: TypeId,
    ctx: &Context,
) -> TypeResult<InferredExpr<'a>> {
    todo!()
}
