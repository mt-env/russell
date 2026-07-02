use crate::frontend::typechecker::types::{TypeValue, TypedExpr};

#[cfg(test)]
mod tests;

pub struct Context {}

impl Context {
    pub fn unify(&mut self, t1: TypeValue, t2: TypeValue) {
        todo!()
    }

    pub fn resolve(&mut self, expr: &TypedExpr) -> TypeValue {
        todo!()
    }

    pub fn lookup(&self, id: &str) -> Option<TypeValue> {
        todo!()
    }

    pub fn extend(&mut self, id: &str, ty: TypeValue) {
        todo!()
    }
}

// TODO figure out how to represent type schemes
// we need union find to unify type variables as well
