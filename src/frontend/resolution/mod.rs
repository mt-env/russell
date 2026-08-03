use crate::frontend::{
    parser::ast::{Defn, ParsedDefn},
    resolution::types::ResolverCtx,
};

pub mod resolve_defn;
pub mod resolve_expr;
pub mod resolve_stmt;
pub mod resolve_type;
pub mod types;

#[cfg(test)]
mod tests;

pub fn resolve(defns: Vec<ParsedDefn>) {
    let mut ctx = ResolverCtx::new();
    init_global_scope(&mut ctx, &defns);
    for defn in defns {
        resolve_defn::resolve_defn(&mut ctx, defn);
    }
}

pub fn init_global_scope<'a>(ctx: &mut ResolverCtx<'a>, defns: &[ParsedDefn<'a>]) {
    let mut has_main = false;
    for defn in defns {
        match &defn.node {
            Defn::Typedef {
                id,
                ty_vars: _,
                arms,
            } => {
                ctx.add_type(id);
                for arm in arms {
                    ctx.add_value(arm.0);
                }
            }
            Defn::Fn { name, .. } => {
                if *name == "main" {
                    has_main = true;
                }
                ctx.add_value(name);
            }
        }
    }
    if !has_main {
        panic!("Program must contain a 'main' function");
    }
    // TODO - resolve collisions in global scope
}
