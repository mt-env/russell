use crate::frontend::{
    parser::ast::{ParsedBinding, Type},
    resolution::types::{ResolvedBinding, ResolvedType, ResolverCtx},
};

pub fn resolve_type(ctx: &mut ResolverCtx, ty: Type) -> ResolvedType {
    match ty {
        Type::Int => ResolvedType::Int,
        Type::Float => ResolvedType::Float,
        Type::Bool => ResolvedType::Bool,
        Type::TypeId(_) => todo!(),
        Type::TypeParam(_) => todo!(),
        Type::TypeApp(_, items) => todo!(),
        Type::Fn(domain, codomain) => todo!(),
    }
}

pub fn resolve_binding<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> ResolvedBinding {
    todo!()
}
