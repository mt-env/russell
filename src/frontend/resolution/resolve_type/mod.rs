use crate::frontend::{
    parser::ast::{ParsedBinding, Type},
    resolution::types::{ResolvedBinding, ResolvedType, ResolverCtx},
};

pub fn resolve_type(ctx: &mut ResolverCtx, ty: Type) -> ResolvedType {
    todo!()
}

pub fn resolve_binding<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> ResolvedBinding {
    todo!()
}
