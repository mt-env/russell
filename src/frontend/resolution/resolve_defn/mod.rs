use crate::frontend::{
    parser::ast::{Defn, ParsedBinding, ParsedDefn, ParsedStmt, Type},
    resolution::{
        resolve_stmt, resolve_type,
        types::{ResolvedDefn, ResolverCtx},
    },
};

#[cfg(test)]
mod tests;

pub fn resolve_defn<'a>(ctx: &mut ResolverCtx<'a>, defn: ParsedDefn<'a>) -> ResolvedDefn {
    match defn.node {
        Defn::Typedef { id, ty_vars, arms } => resolve_typedef(ctx, id, ty_vars, arms),
        Defn::Fn {
            name,
            bindings,
            ret_ty,
            body,
        } => resolve_fn(ctx, name, bindings, ret_ty, body),
    }
}

fn resolve_typedef<'a>(
    ctx: &mut ResolverCtx<'a>,
    id: &'a str,
    ty_vars: Vec<&'a str>,
    arms: Vec<(&'a str, Vec<ParsedBinding<'a>>)>,
) -> ResolvedDefn {
    // fetch type ID from global scope (should have been added in init_global_scope)
    let Some(type_id) = ctx.lookup_type(id) else {
        panic!("Type {} not found in scope", id);
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
        let Some(constructor_id) = ctx.lookup_value(arm.0) else {
            panic!("Constructor {} not found in scope", arm.0);
        };

        ctx.push_scope();
        let mut resolved_bindings = Vec::new();
        for binding in arm.1 {
            ctx.add_value(binding.node.id);
            let resolved_binding = match resolve_type::add_binding_existing_typaram(ctx, binding) {
                Some(val) => val,
                None => panic!("type parameter not found"), // TODO proper error handling
            };
            resolved_bindings.push(resolved_binding);
        }
        ctx.pop_scope();

        resolved_arms.push((constructor_id, resolved_bindings));
    }

    ctx.pop_scope();

    ResolvedDefn::Typedef {
        id: type_id,
        params: resolved_type_params,
        arms: resolved_arms,
    }
}

fn resolve_fn<'a>(
    ctx: &mut ResolverCtx<'a>,
    name: &'a str,
    bindings: Vec<ParsedBinding<'a>>,
    ret_ty: Type<'a>,
    body: Vec<ParsedStmt<'a>>,
) -> ResolvedDefn {
    ctx.push_scope();

    // find function name ID and return type
    let Some(fn_id) = ctx.lookup_value(name) else {
        panic!("Function {} not found in scope", name);
    };

    let ret_ty_id = resolve_type::add_type(ctx, ret_ty);

    // add all bindings to scope
    let mut resolved_bindings = Vec::new();
    for param in bindings {
        resolved_bindings.push(resolve_type::add_binding(ctx, param));
    }

    // resolve function body
    let mut resolved_body = Vec::new();
    for stmt in body {
        resolved_body.push(resolve_stmt::resolve_stmt(ctx, stmt));
    }

    ctx.pop_scope();

    ResolvedDefn::Fn {
        id: fn_id,
        params: resolved_bindings,
        ret_ty: ret_ty_id,
        body: resolved_body,
    }
}
