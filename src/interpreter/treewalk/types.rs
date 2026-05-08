use std::{collections::HashMap, rc::Rc};

use crate::frontend::parser::ast::{Binding, Expr, Stmt, Type};

#[derive(Debug)]
pub(super) struct Env {
    pub next: Option<Rc<Env>>,
    pub binding: (String, Rc<Value>),
}

impl Env {
    pub(super) fn new() -> Env {
        todo!()
    }

    pub(super) fn extend(curr: Rc<Env>, id: String, val: Rc<Value>) -> Rc<Env> {
        Env {
            next: Some(curr),
            binding: (id, val),
        }.into()
    }

    pub(super) fn lookup(&self, key: &str) -> Option<Rc<Value>> {
        if self.binding.0 == key {
            return Some(Rc::clone(&self.binding.1));
        }

        match &self.next {
            Some(env) => env.lookup(key),
            None => None,
        }
    }
}

#[derive(Debug)]
pub(super) enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Closure(Rc<Env>, Binding, Box<Expr>),
    Constructor(String, Type, Vec<Binding>),
    Fn(String, Vec<Binding>, Vec<Stmt>),
    Adt(Type, String, HashMap<String, Rc<Value>>),
}
