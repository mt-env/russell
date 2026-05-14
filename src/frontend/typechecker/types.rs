use crate::frontend::parser::ast::{Defn, Expr, Stmt, Type};

pub(super) type TypedDefn = Defn<TypeValue>;
pub(super) type TypedStmt = Stmt<TypeValue>;
pub(super) type TypedExpr = Expr<TypeValue>;

pub(super) enum TypeValue {
    Int,
    Float,
    Bool,
    Fn(Vec<TypeValue>, Box<TypeValue>), // (arg types, return type)
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
