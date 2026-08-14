use crate::frontend::{
    parser::ast::{ParsedBinding, ParsedType, Type},
    resolution::types::{ResolvedBinding, ResolvedType, ResolverCtx, TypeId, TypeParamId},
};

#[cfg(test)]
mod tests;

pub(super) fn add_type<'a>(ctx: &mut ResolverCtx<'a>, ty: ParsedType<'a>) -> ResolvedType {
    let loc = ty.offset;
    match ty.node {
        Type::Int => ResolvedType::mk_int(loc),
        Type::Float => ResolvedType::mk_float(loc),
        Type::Bool => ResolvedType::mk_bool(loc),
        Type::TypeId(name) => ResolvedType::mk_typeid(loc, resolve_typeid(loc, ctx, name)),
        Type::TypeParam(name) => ResolvedType::mk_typeparam(loc, add_typeparam(ctx, name)),
        Type::TypeApp(type_constr, type_args) => add_typeapp(loc, ctx, type_constr, type_args),
        Type::Fn(domain, codomain) => add_fn(loc, ctx, *domain, *codomain),
    }
}

fn lookup_type<'a>(ctx: &ResolverCtx<'a>, ty: ParsedType<'a>) -> Option<ResolvedType> {
    let loc = ty.offset;
    match ty.node {
        Type::Int => Some(ResolvedType::mk_int(loc)),
        Type::Float => Some(ResolvedType::mk_float(loc)),
        Type::Bool => Some(ResolvedType::mk_bool(loc)),
        Type::TypeId(name) => match ctx.lookup_type(name) {
            Some(id) => Some(ResolvedType::mk_typeid(loc, id)),
            None => None,
        },
        Type::TypeParam(name) => match ctx.lookup_typeparam(name) {
            Some(id) => Some(ResolvedType::mk_typeparam(loc, id)),
            None => None,
        },
        Type::TypeApp(type_constr, type_args) => lookup_typeapp(loc, ctx, type_constr, type_args),
        Type::Fn(domain, codomain) => lookup_fn(loc, ctx, *domain, *codomain),
    }
}

fn resolve_typeid<'a>(offset: usize, ctx: &mut ResolverCtx<'a>, name: &'a str) -> TypeId {
    match ctx.lookup_type(name) {
        Some(val) => val,
        None => {
            ctx.add_unbound_type_error(name, offset);
            ctx.add_type(name) // placeholder type in context to avoid cascading errors
        }
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
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    type_constr: &'a str,
    type_args: Vec<ParsedType<'a>>,
) -> ResolvedType {
    let resolved_type_constr = resolve_typeid(offset, ctx, type_constr);
    let mut resolved_type_args = Vec::with_capacity(type_args.len());
    for arg in type_args {
        resolved_type_args.push(add_type(ctx, arg));
    }
    ResolvedType::mk_typeapp(offset, resolved_type_constr, resolved_type_args)
}

fn lookup_typeapp<'a>(
    offset: usize,
    ctx: &ResolverCtx<'a>,
    type_constr: &'a str,
    type_args: Vec<ParsedType<'a>>,
) -> Option<ResolvedType> {
    let resolved_type_constr = ctx.lookup_type(type_constr)?;
    let mut resolved_type_args = Vec::with_capacity(type_args.len());
    for arg in type_args {
        resolved_type_args.push(lookup_type(ctx, arg)?);
    }
    Some(ResolvedType::mk_typeapp(
        offset,
        resolved_type_constr,
        resolved_type_args,
    ))
}

fn add_fn<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    domain: ParsedType<'a>,
    codomain: ParsedType<'a>,
) -> ResolvedType {
    let domain = add_type(ctx, domain);
    let codomain = add_type(ctx, codomain);
    ResolvedType::mk_fn(offset, domain, codomain)
}

fn lookup_fn<'a>(
    offset: usize,
    ctx: &ResolverCtx<'a>,
    domain: ParsedType<'a>,
    codomain: ParsedType<'a>,
) -> Option<ResolvedType> {
    let domain = lookup_type(ctx, domain)?;
    let codomain = lookup_type(ctx, codomain)?;
    Some(ResolvedType::mk_fn(offset, domain, codomain))
}

// resolves a binding, adds the value to the context, and returns a ResolvedBinding
pub(super) fn add_binding<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> ResolvedBinding {
    let value_id = ctx.add_value(binding.node.id);
    let resolved_type = add_type(ctx, binding.node.typ);
    ResolvedBinding::new(binding.offset, value_id, resolved_type)
}

pub(super) fn add_binding_no_shadowing<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> ResolvedBinding {
    let value_id = ctx.add_value_nodup(binding.node.id, binding.offset);
    let resolved_type = add_type(ctx, binding.node.typ);
    ResolvedBinding::new(binding.offset, value_id, resolved_type)
}

// adds a binding but won't automatically create a new typeparam
// returns none if the typeparam doesn't exist in the context
pub(super) fn add_binding_existing_typaram<'a>(
    ctx: &mut ResolverCtx<'a>,
    binding: ParsedBinding<'a>,
) -> Option<ResolvedBinding> {
    let resolved_type = lookup_type(ctx, binding.node.typ)?;
    let value_id = ctx.add_value_nodup(binding.node.id, binding.offset);
    Some(ResolvedBinding::new(
        binding.offset,
        value_id,
        resolved_type,
    ))
}
