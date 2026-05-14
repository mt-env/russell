use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::frontend::parser::ast::{Binding, ParsedExpr, ParsedStmt, Type};

#[derive(Debug)]
pub(super) enum Env {
    Global(HashMap<String, Rc<Value>>),
    Local {
        next: Rc<Env>,
        global: Rc<Env>,
        binding: (String, Rc<Value>),
    },
}

impl Env {
    pub(super) fn extend(curr: Rc<Env>, id: String, val: Rc<Value>) -> Rc<Env> {
        let global = curr.global();
        Env::Local {
            global,
            next: curr,
            binding: (id, val),
        }.into()
    }

    pub(super) fn global(self: &Rc<Self>) -> Rc<Self> {
        match self.as_ref() {
            Env::Global(_) => Rc::clone(self),
            Env::Local { global, .. } => Rc::clone(global),
        }
    }

    pub(super) fn lookup(&self, key: &str) -> Option<Rc<Value>> {
        match self {
            Env::Global(map) => map.get(key).map(Rc::clone),
            Env::Local { next, binding, .. } => {
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
