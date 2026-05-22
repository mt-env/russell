use std::{collections::HashMap, rc::Rc};

use crate::frontend::parser::ast::{Defn, Expr, Stmt, Type};

pub type TypeResult<T> = Result<T, TypeError>;
pub type TypedDefn = Defn<TypeValue>;
pub type TypedStmt = Stmt<TypeValue>;
pub type TypedExpr = Expr<TypeValue>;

pub struct TypeError {
    expected: TypeValue,
    actual: TypeValue,
    message: String,
}

pub enum Env {
    Global(HashMap<String, TypeValue>),
    Local {
        next: Rc<Env>,
        binding: (String, TypeValue),
    }
}

pub enum TypeValue {
    Int,
    Float,
    Bool,
    Fn(Vec<TypeValue>, Box<TypeValue>), // (arg types, return type)
    Closure(Box<TypeValue>, Box<TypeValue>), // (arg type, return type)
    Adt(String), // nominal type
    Var(Box<Option<TypeValue>>), // type variable for inference
}

impl From<Type> for TypeValue {
    fn from(value: Type) -> Self {
        match value {
            Type::Int => TypeValue::Int,
            Type::Float => TypeValue::Float,
            Type::Bool => TypeValue::Bool,
            Type::TypeId(_) => todo!(),
            Type::Fn(arg, body) => TypeValue::Fn(
                vec![(*arg).into()],
                Box::new((*body).into())
            ),
        }
    }
}
