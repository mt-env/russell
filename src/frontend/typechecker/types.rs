use crate::frontend::parser::ast::{Expr, Type};

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

pub(super) struct TypedExpr {
    pub ty: TypeValue,
    pub expr: Expr,
}

impl TypedExpr {
    pub fn new(ty: TypeValue, expr: Expr) -> Self {
        Self { ty, expr }
    }
}

