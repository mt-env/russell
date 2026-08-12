use crate::frontend::{
    parser::ast::{Defn, ParsedBinding, ParsedConstructorArm, ParsedDefn, ParsedStmt, ParsedType},
    resolution::{
        resolve_stmt, resolve_type,
        types::{ResolvedConstructorArm, ResolvedDefn, ResolverCtx},
    },
};

#[cfg(test)]
mod tests;

pub fn resolve_defn<'a>(ctx: &mut ResolverCtx<'a>, defn: ParsedDefn<'a>) -> ResolvedDefn {
    let loc = defn.offset;
    match defn.node {
        Defn::Typedef { id, ty_vars, arms } => resolve_typedef(loc, ctx, id, ty_vars, arms),
        Defn::Fn {
            name,
            bindings,
            ret_ty,
            body,
        } => resolve_fn(loc, ctx, name, bindings, ret_ty, body),
    }
}

fn resolve_typedef<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    id: &'a str,
    ty_vars: Vec<&'a str>,
    arms: Vec<ParsedConstructorArm<'a>>,
) -> ResolvedDefn {
    // fetch type ID from global scope (should have been added in init_global_scope)
    let Some(type_id) = ctx.lookup_type(id) else {
        unreachable!(
            "COMPILER BUG: cannot find type {}. please report this error",
            id
        );
    };

    // set up generic tyvars
    ctx.push_scope();

    let mut resolved_type_params = Vec::new();
    for ty_var in ty_vars {
        resolved_type_params.push(ctx.add_typeparam(ty_var));
    }

    // validate each arm
    let mut resolved_arms = Vec::new();
    for arm in arms {
        let Some(constructor_id) = ctx.lookup_value(arm.node.id) else {
            unreachable!(
                "COMPILER BUG: cannot find constructor {}. please report this error",
                arm.node.id
            );
        };

        ctx.push_scope();
        let mut resolved_bindings = Vec::new();
        for binding in arm.node.bindings {
            let loc = binding.offset;
            let name = binding.node.id;
            ctx.add_value(name);
            let resolved_binding =
            // TODO - nuke the clone if possible
                match resolve_type::add_binding_existing_typaram(ctx, binding.clone()) {
                    Some(val) => val,
                    None => {
                        ctx.add_unbound_type_error(name, loc);
                        resolve_type::add_binding_no_shadowing(ctx, binding) // placeholder binding
                    }
                };
            resolved_bindings.push(resolved_binding);
        }
        ctx.pop_scope();

        resolved_arms.push(ResolvedConstructorArm::new(
            arm.offset,
            constructor_id,
            resolved_bindings,
        ));
    }

    ctx.pop_scope();

    ResolvedDefn::mk_typedef(offset, type_id, resolved_type_params, resolved_arms)
}

fn resolve_fn<'a>(
    offset: usize,
    ctx: &mut ResolverCtx<'a>,
    name: &'a str,
    bindings: Vec<ParsedBinding<'a>>,
    ret_ty: ParsedType<'a>,
    body: Vec<ParsedStmt<'a>>,
) -> ResolvedDefn {
    ctx.push_scope();

    // find function name ID and return type
    let Some(fn_id) = ctx.lookup_value(name) else {
        unreachable!(
            "COMPILER BUG: cannot find function {}. please report this error",
            name
        );
    };

    let resolved_ret_ty = resolve_type::add_type(ctx, ret_ty);

    // add all bindings to scope
    let mut resolved_bindings = Vec::new();
    for param in bindings {
        resolved_bindings.push(resolve_type::add_binding_no_shadowing(ctx, param));
    }

    // resolve function body
    let mut resolved_body = Vec::new();
    for stmt in body {
        resolved_body.push(resolve_stmt::resolve_stmt(ctx, stmt));
    }

    ctx.pop_scope();

    ResolvedDefn::mk_fn(
        offset,
        fn_id,
        resolved_bindings,
        resolved_ret_ty,
        resolved_body,
    )
}
