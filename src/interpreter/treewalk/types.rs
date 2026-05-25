use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::frontend::parser::ast::{Binding, ParsedExpr, ParsedStmt, Type};

#[derive(Debug)]
pub(super) enum Env<'a> {
    Global(HashMap<&'a str, Rc<Value<'a>>>),
    Local {
        next: Rc<Env<'a>>,
        global: Rc<Env<'a>>,
        binding: (&'a str, Rc<Value<'a>>),
    },
}

impl<'a> Env<'a> {
    pub(super) fn extend(curr: Rc<Env<'a>>, id: &'a str, val: Rc<Value<'a>>) -> Rc<Env<'a>> {
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

    pub(super) fn lookup(&self, key: &str) -> Option<Rc<Value<'a>>> {
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
pub(super) enum Value<'a> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Closure(Rc<Env<'a>>, Binding<'a>, Box<ParsedExpr<'a>>),
    Constructor(&'a str, Type<'a>, Vec<Binding<'a>>),
    Fn(&'a str, Vec<Binding<'a>>, Vec<ParsedStmt<'a>>),
    Adt(Type<'a>, &'a str, HashMap<&'a str, Rc<Value<'a>>>),
}

impl Display for Value<'_> {
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
