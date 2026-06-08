use std::{collections::HashMap, rc::Rc};

use crate::frontend::parser::ast::{Defn, Expr, ExprKind, Stmt, Type};

pub type TypeResult<T> = Result<T, TypeError>;
pub(super) type TypedDefn<'a> = Defn<'a, TypeValue>;
pub(super) type TypedStmt<'a> = Stmt<'a, TypeValue>;
pub(super) type TypedExpr<'a> = Expr<'a, TypeValue>;

impl<'a> TypedExpr<'a> {
    pub fn new(ann: TypeValue, kind: ExprKind<'a, TypeValue>) -> Self {
        Self { ann, kind }
    }
}

pub struct TypeError {
    pub expected: TypeValue,
    pub actual: TypeValue,
    pub offset: usize,
}

pub enum Env {
    Global(HashMap<String, TypeValue>),
    Local {
        next: Rc<Env>,
        binding: (String, TypeValue),
    },
}

pub enum TypeValue {
    Int,
    Float,
    Bool,
    Fn(Vec<TypeValue>, Box<TypeValue>),      // (arg types, return type)
    Closure(Box<TypeValue>, Box<TypeValue>), // (arg type, return type)
    Adt(String),                             // nominal type
    Var(Box<Option<TypeValue>>),             // type variable for inference
}

impl From<Type<'_>> for TypeValue {
    fn from(value: Type) -> Self {
        match value {
            Type::Int => TypeValue::Int,
            Type::Float => TypeValue::Float,
            Type::Bool => TypeValue::Bool,
            Type::TypeId(_) => todo!(),
            Type::Fn(arg, body) => TypeValue::Fn(vec![(*arg).into()], Box::new((*body).into())),
        }
    }
}
