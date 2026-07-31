use crate::frontend::{
    parser::ast::{ParsedBinding, Type},
    resolution::types::{ResolvedBinding, ResolvedType, ResolverCtx, TypeId, TypeParamId},
};

pub fn resolve_type<'a>(ctx: &mut ResolverCtx<'a>, ty: Type<'a>) -> ResolvedType {
    match ty {
        Type::Int => ResolvedType::Int,
        Type::Float => ResolvedType::Float,
        Type::Bool => ResolvedType::Bool,
        Type::TypeId(name) => ResolvedType::TypeId(resolve_typeid(ctx, name)),
        Type::TypeParam(name) => ResolvedType::TypeParam(resolve_typeparam(ctx, name)),
        Type::TypeApp(type_constr, type_args) => resolve_typeapp(ctx, type_constr, type_args),
        Type::Fn(domain, codomain) => resolve_fn(ctx, *domain, *codomain),
    }
}

fn resolve_typeid(ctx: &ResolverCtx, name: &str) -> TypeId {
    match ctx.lookup_type(name) {
        Some(val) => val,
        None => panic!("TypeId {} not found in context", name),
    }
}

// this adds in a type param if it doesn't exist
// TODO is that a bad choice?
fn resolve_typeparam<'a>(ctx: &mut ResolverCtx<'a>, name: &'a str) -> TypeParamId {
    match ctx.lookup_typeparam(name) {
        Some(val) => val,
        None => ctx.add_typeparam(name),
    }
}

fn resolve_typeapp<'a>(
    ctx: &mut ResolverCtx<'a>,
    type_constr: &'a str,
    type_args: Vec<Type<'a>>,
) -> ResolvedType {
    let resolved_type_constr = resolve_typeid(ctx, type_constr);
    let mut resolved_type_args = Vec::with_capacity(type_args.len());
    for arg in type_args {
        resolved_type_args.push(resolve_type(ctx, arg));
    }
    ResolvedType::TypeApp(resolved_type_constr, resolved_type_args)
}

fn resolve_fn<'a>(ctx: &mut ResolverCtx<'a>, domain: Type<'a>, codomain: Type<'a>) -> ResolvedType {
    let domain = resolve_type(ctx, domain);
    let codomain = resolve_type(ctx, codomain);
    ResolvedType::Fn(Box::new(domain), Box::new(codomain))
}

// resolves a binding, adds the value to the context, and returns a ResolvedBinding
pub fn resolve_binding<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> ResolvedBinding {
    let value_id = ctx.add_value(binding.node.id);
    let resolved_type = resolve_type(ctx, binding.node.typ);
    ResolvedBinding::new(value_id, resolved_type)
}
