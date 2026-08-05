use crate::frontend::{
    parser::ast::{Defn, ParsedDefn},
    resolution::types::{ResolvedDefn, ResolverCtx},
};

pub mod resolve_defn;
pub mod resolve_expr;
pub mod resolve_stmt;
pub mod resolve_type;
pub mod types;

#[cfg(test)]
mod tests;

pub fn resolve(defns: Vec<ParsedDefn>) -> Vec<ResolvedDefn> {
    let mut ctx = ResolverCtx::new();
    init_global_scope(&mut ctx, &defns);
    let mut resolved_defns = Vec::new();
    for defn in defns {
        resolved_defns.push(resolve_defn::resolve_defn(&mut ctx, defn));
    }
    resolved_defns
}

pub fn init_global_scope<'a>(ctx: &mut ResolverCtx<'a>, defns: &[ParsedDefn<'a>]) {
    let mut has_main = false;
    for defn in defns {
        match &defn.node {
            Defn::Typedef { id, arms, .. } => {
                ctx.add_type_nodup(id);
                for arm in arms {
                    ctx.add_value_nodup(arm.0);
                }
            }
            Defn::Fn { name, .. } => {
                if *name == "main" {
                    has_main = true;
                }
                ctx.add_value_nodup(name);
            }
        }
    }
    if !has_main {
        panic!("Program must contain a 'main' function");
    }
    // TODO - resolve collisions in global scope
}
