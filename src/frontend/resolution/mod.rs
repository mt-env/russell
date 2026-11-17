use crate::frontend::{
    parser::ast::{Defn, ParsedDefn},
    resolution::types::{
        ResolveErrorKind, ResolveResult, ResolveSuccess, ResolverCtx,
    },
    types::Spanned,
};

pub mod resolve_defn;
pub mod resolve_expr;
pub mod resolve_stmt;
pub mod resolve_type;
pub mod types;

#[cfg(test)]
mod tests;

pub fn resolve<'src>(defns: Vec<ParsedDefn<'src>>) -> ResolveResult<'src, ResolveSuccess<'src>> {
    let mut ctx = ResolverCtx::new();
    init_global_scope(&mut ctx, &defns)?;
    let mut resolved_defns = Vec::new();
    for defn in defns {
        resolved_defns.push(resolve_defn::resolve_defn(&mut ctx, defn));
    }

    let Some(main_id) = ctx.lookup_value("main") else {
        unreachable!(
            "COMPILER BUG: failed to report missing main function. please report this error"
        );
    };
    Ok(ResolveSuccess {
        resolved_defns,
        main_id,
        id_to_name: ctx.ids,
    })
}

fn init_global_scope<'src>(
    ctx: &mut ResolverCtx<'src>,
    defns: &[ParsedDefn<'src>],
) -> ResolveResult<'src, ()> {
    let mut has_main = false;
    for defn in defns {
        match &defn.node {
            Defn::Typedef { id, arms, .. } => {
                ctx.add_type_nodup(id, defn.offset);
                for arm in arms {
                    ctx.add_value_nodup(arm.node.id, arm.offset);
                }
            }
            Defn::Fn { name, .. } => {
                if *name == "main" {
                    has_main = true;
                }
                ctx.add_value_nodup(name, defn.offset);
            }
        }
    }

    if !has_main {
        return Err(Spanned {
            offset: 0, // TODO placeholder value - maybe should be end of program? that'd be
            // annoying to thread through everything though
            node: ResolveErrorKind::MissingMainFn,
        });
    }

    Ok(())
}
