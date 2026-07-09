use std::collections::HashMap;

use crate::frontend::{
    parser::ast::{Expr, ExprKind, SpannedExpr},
    typechecker::types::TypeValue,
};

#[cfg(test)]
mod tests;

pub type InferredExpr<'a> = SpannedExpr<'a, TypeId>;

impl<'a> InferredExpr<'a> {
    pub fn new(offset: usize, ann: TypeId, kind: ExprKind<'a, TypeId>) -> Self {
        Self {
            offset,
            node: Expr { ann, kind },
        }
    }

    pub fn ty(&self) -> TypeId {
        self.node.ann
    }
}

#[derive(Clone, Copy)]
pub struct TypeId(usize);

pub struct Context {
    ty_vars: usize,
}

impl Context {
    pub fn unify(&mut self, t1: TypeId, t2: TypeId) {
        todo!()
    }

    pub fn int_id(&self) -> TypeId {
        todo!()
    }

    pub fn bool_id(&self) -> TypeId {
        todo!()
    }

    pub fn float_id(&self) -> TypeId {
        todo!()
    }

    pub fn resolve(&mut self, expr: &InferredExpr) -> TypeValue {
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

struct Env {
    scopes: Vec<Scope>,
}

struct Scope {
    values: HashMap<String, TypeId>,
}

// TODO figure out how to represent type schemes
// we need union find to unify type variables as well
