use crate::frontend::{
    parser::ast::{ParsedBinding, ParsedType, Type},
    resolution::types::{
        ResolveErrorKind, ResolveResult, ResolvedBinding, ResolvedType, ResolverCtx, TypeId,
        TypeParamId,
    },
    types::Spanned,
};

#[cfg(test)]
mod tests;

pub(super) fn add_type<'src>(
    ctx: &mut ResolverCtx<'src>,
    ty: ParsedType<'src>,
) -> ResolveResult<'src, ResolvedType> {
    let loc = ty.offset;
    match ty.node {
        Type::Int => Ok(ResolvedType::mk_int(loc)),
        Type::Float => Ok(ResolvedType::mk_float(loc)),
        Type::Bool => Ok(ResolvedType::mk_bool(loc)),
        Type::TypeId(name) => Ok(ResolvedType::mk_typeid(
            loc,
            resolve_typeid(loc, ctx, name)?,
        )),
        Type::TypeParam(name) => Ok(ResolvedType::mk_typeparam(loc, add_typeparam(ctx, name))),
        Type::TypeApp(type_constr, type_args) => add_typeapp(loc, ctx, type_constr, type_args),
        Type::Fn(domain, codomain) => add_fn(loc, ctx, *domain, *codomain),
    }
}

fn lookup_type<'src>(
    ctx: &ResolverCtx<'src>,
    ty: ParsedType<'src>,
) -> ResolveResult<'src, ResolvedType> {
    let loc = ty.offset;
    match ty.node {
        Type::Int => Ok(ResolvedType::mk_int(loc)),
        Type::Float => Ok(ResolvedType::mk_float(loc)),
        Type::Bool => Ok(ResolvedType::mk_bool(loc)),
        Type::TypeId(name) => ctx.lookup_type(name).map_or(
            Err(Spanned {
                offset: loc,
                node: ResolveErrorKind::UnknownType { name },
            }),
            |id| Ok(ResolvedType::mk_typeid(loc, id)),
        ),
        Type::TypeParam(name) => ctx.lookup_typeparam(name).map_or(
            Err(Spanned {
                offset: loc,
                node: ResolveErrorKind::UnknownType { name },
            }),
            |id| Ok(ResolvedType::mk_typeparam(loc, id)),
        ),
        Type::TypeApp(type_constr, type_args) => lookup_typeapp(loc, ctx, type_constr, type_args),
        Type::Fn(domain, codomain) => lookup_fn(loc, ctx, *domain, *codomain),
    }
}

fn resolve_typeid<'src>(
    offset: usize,
    ctx: &ResolverCtx<'src>,
    name: &'src str,
) -> ResolveResult<'src, TypeId> {
    ctx.lookup_type(name).ok_or(Spanned {
        offset,
        node: ResolveErrorKind::UnknownType { name },
    })
}

// this adds in a type param if it doesn't exist
fn add_typeparam<'src>(ctx: &mut ResolverCtx<'src>, name: &'src str) -> TypeParamId {
    ctx.lookup_typeparam(name)
        .unwrap_or_else(|| ctx.add_typeparam(name))
}

fn add_typeapp<'src>(
    offset: usize,
    ctx: &mut ResolverCtx<'src>,
    type_constr: &'src str,
    type_args: Vec<ParsedType<'src>>,
) -> ResolveResult<'src, ResolvedType> {
    let resolved_type_constr = resolve_typeid(offset, ctx, type_constr)?;
    let mut resolved_type_args = Vec::with_capacity(type_args.len());
    for arg in type_args {
        resolved_type_args.push(add_type(ctx, arg)?);
    }
    Ok(ResolvedType::mk_typeapp(
        offset,
        resolved_type_constr,
        resolved_type_args,
    ))
}

fn lookup_typeapp<'src>(
    offset: usize,
    ctx: &ResolverCtx<'src>,
    type_constr: &'src str,
    type_args: Vec<ParsedType<'src>>,
) -> ResolveResult<'src, ResolvedType> {
    let resolved_type_constr = ctx.lookup_type(type_constr).ok_or(Spanned {
        offset,
        node: ResolveErrorKind::UnknownType { name: type_constr },
    })?;
    let mut resolved_type_args = Vec::with_capacity(type_args.len());
    for arg in type_args {
        resolved_type_args.push(lookup_type(ctx, arg)?);
    }
    Ok(ResolvedType::mk_typeapp(
        offset,
        resolved_type_constr,
        resolved_type_args,
    ))
}

fn add_fn<'src>(
    offset: usize,
    ctx: &mut ResolverCtx<'src>,
    domain: ParsedType<'src>,
    codomain: ParsedType<'src>,
) -> ResolveResult<'src, ResolvedType> {
    let domain = add_type(ctx, domain)?;
    let codomain = add_type(ctx, codomain)?;
    Ok(ResolvedType::mk_fn(offset, domain, codomain))
}

fn lookup_fn<'src>(
    offset: usize,
    ctx: &ResolverCtx<'src>,
    domain: ParsedType<'src>,
    codomain: ParsedType<'src>,
) -> ResolveResult<'src, ResolvedType> {
    let domain = lookup_type(ctx, domain)?;
    let codomain = lookup_type(ctx, codomain)?;
    Ok(ResolvedType::mk_fn(offset, domain, codomain))
}

// resolves a binding, adds the value to the context, and returns a ResolvedBinding
pub(super) fn add_binding<'src>(
    ctx: &mut ResolverCtx<'src>,
    binding: ParsedBinding<'src>,
) -> ResolveResult<'src, ResolvedBinding> {
    let value_id = ctx.add_value(binding.node.id);
    let resolved_type = add_type(ctx, binding.node.typ)?;
    Ok(ResolvedBinding::new(
        binding.offset,
        value_id,
        resolved_type,
    ))
}

pub(super) fn add_binding_no_shadowing<'src>(
    ctx: &mut ResolverCtx<'src>,
    binding: ParsedBinding<'src>,
) -> ResolveResult<'src, ResolvedBinding> {
    let value_id = ctx.add_value_nodup(binding.node.id, binding.offset)?;
    let resolved_type = add_type(ctx, binding.node.typ)?;
    Ok(ResolvedBinding::new(
        binding.offset,
        value_id,
        resolved_type,
    ))
}

// adds a binding but won't automatically create a new typeparam
// returns none if the typeparam doesn't exist in the context
pub(super) fn add_binding_existing_typaram<'src>(
    ctx: &mut ResolverCtx<'src>,
    binding: ParsedBinding<'src>,
) -> ResolveResult<'src, ResolvedBinding> {
    let resolved_type = lookup_type(ctx, binding.node.typ)?;
    let value_id = ctx.add_value_nodup(binding.node.id, binding.offset)?;
    Ok(ResolvedBinding::new(
        binding.offset,
        value_id,
        resolved_type,
    ))
}
