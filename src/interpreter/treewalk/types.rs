use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::frontend::parser::ast::{Binding, ParsedExpr, ParsedStmt, Type};

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
    Closure(Rc<Env>, Binding, Box<ParsedExpr>),
    Constructor(String, Type, Vec<Binding>),
    Fn(String, Vec<Binding>, Vec<ParsedStmt>),
    Adt(Type, String, HashMap<String, Rc<Value>>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(num) => write!(f, "{num}"),
            Value::Float(num) => write!(f, "{num:?}"),
            Value::Bool(val) => write!(f, "{val}"),
            Value::Closure(_, binding, expr) => write!(f, "<function ({binding}) -> {expr}>"),
            Value::Constructor(name, _, fields) => {
                let joined = fields.iter().map(
                    |b| b.to_string()
                ).collect::<Vec<_>>().join(", ");
                write!(f, "<constructor {name} {joined}>")
            }
            Value::Fn(name, _, _) => write!(f, "<function {name}>"),
            Value::Adt(adt_type, name, data) => {
                let fields = data.iter().map(
                    |(k, v)| format!("{k}: {v}")
                ).collect::<Vec<_>>().join(", ");
                write!(f, "<ADT {adt_type} {name} {{{fields}}}>")
            }
        }
    }
}
