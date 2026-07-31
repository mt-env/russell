use crate::frontend::{
    parser::ast::{ParsedBinding, Type},
    resolution::types::{ResolvedBinding, ResolvedType, ResolverCtx},
};

pub fn resolve_type<'a>(ctx: &mut ResolverCtx<'a>, ty: Type<'a>) -> ResolvedType {
    match ty {
        Type::Int => ResolvedType::Int,
        Type::Float => ResolvedType::Float,
        Type::Bool => ResolvedType::Bool,
        Type::TypeId(name) => resolve_typeid(ctx, name),
        Type::TypeParam(name) => resolve_typeparam(ctx, name),
        Type::TypeApp(_, items) => todo!(),
        Type::Fn(domain, codomain) => resolve_fn(ctx, *domain, *codomain),
    }
}

fn resolve_typeid(ctx: &ResolverCtx, name: &str) -> ResolvedType {
    let Some(id) = ctx.lookup_type(name) else {
        panic!("Unbound type: {}", name);
    };
    ResolvedType::TypeId(id)
}

fn resolve_typeparam<'a>(ctx: &mut ResolverCtx<'a>, name: &'a str) -> ResolvedType {
    match ctx.lookup_typeparam(name) {
        Some(val) => ResolvedType::TypeParam(val),
        None => ResolvedType::TypeParam(ctx.add_typeparam(name)),
    }
}

fn resolve_fn<'a>(ctx: &mut ResolverCtx<'a>, domain: Type<'a>, codomain: Type<'a>) -> ResolvedType {
    let domain = resolve_type(ctx, domain);
    let codomain = resolve_type(ctx, codomain);
    ResolvedType::Fn(Box::new(domain), Box::new(codomain))
}

pub fn resolve_binding<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> ResolvedBinding {
    todo!()
}
