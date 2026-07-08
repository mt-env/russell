use crate::frontend::typechecker::types::{TypeValue, TypedExpr};

#[cfg(test)]
mod tests;

pub struct Context {
    ty_vars: usize,
}

impl Context {
    pub fn unify(&mut self, t1: TypeValue, t2: TypeValue) {
        let a = t1;
        let b = t2;
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

    pub fn new_tyvar(&mut self) -> TypeValue {
        let tyvar = TypeValue::Var(self.ty_vars);
        self.ty_vars += 1;
        tyvar
    }
}

// TODO figure out how to represent type schemes
// we need union find to unify type variables as well
