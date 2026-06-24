use std::{collections::HashMap, rc::Rc};

use crate::frontend::parser::ast::{Expr, ExprKind, SpannedDefn, SpannedExpr, SpannedStmt, Type};

pub type TypeResult<T> = Result<T, TypeError>;
pub(super) type TypedDefn<'a> = SpannedDefn<'a, TypeValue>;
pub(super) type TypedStmt<'a> = SpannedStmt<'a, TypeValue>;
pub(super) type TypedExpr<'a> = SpannedExpr<'a, TypeValue>;

impl<'a> TypedStmt<'a> {
    pub fn make_let(offset: usize, id: &'a str, expr: TypedExpr<'a>) -> Self {
        todo!()
    }

    pub fn make_read(offset: usize, ty: Type<'a>, id: &'a str) -> Self {
        todo!()
    }

    pub fn make_echo(offset: usize, ty: Type<'a>, expr: TypedExpr<'a>) -> Self {
        todo!()
    }

    pub fn make_return(offset: usize, expr: TypedExpr<'a>) -> Self {
        todo!()
    }
}

impl<'a> TypedExpr<'a> {
    pub fn new(offset: usize, ann: TypeValue, kind: ExprKind<'a, TypeValue>) -> Self {
        Self {
            offset,
            node: Expr { ann, kind },
        }
    }

    pub fn ty(&self) -> TypeValue {
        self.node.ann.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
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

impl Env {
    pub fn lookup(&self, id: &str) -> Option<TypeValue> {
        todo!()
    }

    pub fn extend(&mut self, id: &str, ty: TypeValue) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeValue {
    Int,
    Float,
    Bool,
    Fn(Vec<TypeValue>, Box<TypeValue>), // (arg types, return type)
    Closure(Box<TypeValue>, Box<TypeValue>), // (arg type, return type)
    Adt(String),                        // nominal type
    Var(Box<Option<TypeValue>>),        // type variable for inference
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
