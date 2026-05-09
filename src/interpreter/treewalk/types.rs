use std::{collections::HashMap, rc::Rc};

use crate::frontend::parser::ast::{Binding, Expr, Stmt, Type};

#[derive(Debug)]
pub(super) enum Env {
    Mt, // empy :3
    Cons {
        next: Rc<Env>,
        binding: (String, Rc<Value>),
    },
}

impl Env {
    pub(super) fn new() -> Env {
        Env::Mt
    }

    pub(super) fn extend(curr: Rc<Env>, id: String, val: Rc<Value>) -> Rc<Env> {
        Env::Cons {
            next: curr,
            binding: (id, val),
        }.into()
    }

    pub(super) fn lookup(&self, key: &str) -> Option<Rc<Value>> {
        match self {
            Env::Mt => None,
            Env::Cons { next, binding } => {
                if binding.0 == key {
                    Some(Rc::clone(&binding.1))
                } else {
                    next.lookup(key)
                }
            }
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
