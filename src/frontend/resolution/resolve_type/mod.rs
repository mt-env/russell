use crate::frontend::{
    parser::ast::{ParsedBinding, Type},
    resolution::types::{ResolvedBinding, ResolvedType, ResolverCtx, TypeId, TypeParamId},
};

#[cfg(test)]
mod tests;

pub(super) fn add_type<'a>(ctx: &mut ResolverCtx<'a>, ty: Type<'a>) -> ResolvedType {
    match ty {
        Type::Int => ResolvedType::Int,
        Type::Float => ResolvedType::Float,
        Type::Bool => ResolvedType::Bool,
        Type::TypeId(name) => ResolvedType::TypeId(resolve_typeid(ctx, name)),
        Type::TypeParam(name) => ResolvedType::TypeParam(add_typeparam(ctx, name)),
        Type::TypeApp(type_constr, type_args) => add_typeapp(ctx, type_constr, type_args),
        Type::Fn(domain, codomain) => add_fn(ctx, *domain, *codomain),
    }
}

fn lookup_type<'a>(ctx: &ResolverCtx<'a>, ty: Type<'a>) -> Option<ResolvedType> {
    match ty {
        Type::Int => Some(ResolvedType::Int),
        Type::Float => Some(ResolvedType::Float),
        Type::Bool => Some(ResolvedType::Bool),
        Type::TypeId(name) => ctx.lookup_type(name).map(ResolvedType::TypeId),
        Type::TypeParam(name) => ctx.lookup_typeparam(name).map(ResolvedType::TypeParam),
        Type::TypeApp(type_constr, type_args) => lookup_typeapp(ctx, type_constr, type_args),
        Type::Fn(domain, codomain) => lookup_fn(ctx, *domain, *codomain),
    }
}

fn resolve_typeid(ctx: &ResolverCtx, name: &str) -> TypeId {
    match ctx.lookup_type(name) {
        Some(val) => val,
        None => panic!("TypeId {} not found in context", name),
    }
}

// this adds in a type param if it doesn't exist
fn add_typeparam<'a>(ctx: &mut ResolverCtx<'a>, name: &'a str) -> TypeParamId {
    match ctx.lookup_typeparam(name) {
        Some(val) => val,
        None => ctx.add_typeparam(name),
    }
}

fn add_typeapp<'a>(
    ctx: &mut ResolverCtx<'a>,
    type_constr: &'a str,
    type_args: Vec<Type<'a>>,
) -> ResolvedType {
    let resolved_type_constr = resolve_typeid(ctx, type_constr);
    let mut resolved_type_args = Vec::with_capacity(type_args.len());
    for arg in type_args {
        resolved_type_args.push(add_type(ctx, arg));
    }
    ResolvedType::TypeApp(resolved_type_constr, resolved_type_args)
}

fn lookup_typeapp<'a>(
    ctx: &ResolverCtx<'a>,
    type_constr: &'a str,
    type_args: Vec<Type<'a>>,
) -> Option<ResolvedType> {
    let resolved_type_constr = ctx.lookup_type(type_constr)?;
    let mut resolved_type_args = Vec::with_capacity(type_args.len());
    for arg in type_args {
        resolved_type_args.push(lookup_type(ctx, arg)?);
    }
    Some(ResolvedType::TypeApp(
        resolved_type_constr,
        resolved_type_args,
    ))
}

fn add_fn<'a>(ctx: &mut ResolverCtx<'a>, domain: Type<'a>, codomain: Type<'a>) -> ResolvedType {
    let domain = add_type(ctx, domain);
    let codomain = add_type(ctx, codomain);
    ResolvedType::Fn(Box::new(domain), Box::new(codomain))
}

fn lookup_fn<'a>(
    ctx: &ResolverCtx<'a>,
    domain: Type<'a>,
    codomain: Type<'a>,
) -> Option<ResolvedType> {
    let domain = lookup_type(ctx, domain)?;
    let codomain = lookup_type(ctx, codomain)?;
    Some(ResolvedType::Fn(Box::new(domain), Box::new(codomain)))
}

// resolves a binding, adds the value to the context, and returns a ResolvedBinding
pub(super) fn add_binding<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> ResolvedBinding {
    let value_id = ctx.add_value(binding.node.id);
    let resolved_type = add_type(ctx, binding.node.typ);
    ResolvedBinding::new(value_id, resolved_type)
}

pub(super) fn add_binding_no_shadowing<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> ResolvedBinding {
    let value_id = ctx.add_value_nodup(binding.node.id);
    let resolved_type = add_type(ctx, binding.node.typ);
    ResolvedBinding::new(value_id, resolved_type)
}

// adds a binding but won't automatically create a new typeparam
// returns none if the typeparam doesn't exist in the context
pub(super) fn add_binding_existing_typaram<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> Option<ResolvedBinding> {
    let value_id = ctx.add_value(binding.node.id);
    let resolved_type = lookup_type(ctx, binding.node.typ)?;
    Some(ResolvedBinding::new(value_id, resolved_type))
}
