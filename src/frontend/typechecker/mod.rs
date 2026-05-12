use crate::frontend::parser::ast::Defn;

pub mod types;
pub mod typecheck_expr;
pub mod typecheck_fn;


fn typecheck(defns: Vec<Defn>) -> Vec<TypedDefn> {
    todo!()
}
